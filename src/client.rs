//! Payrix API client implementation.

use crate::entity::EntityType;
use crate::error::{Error, PayrixApiError, Result};
use crate::rate_limiter::RateLimiter;
use crate::search::build_expand_query;
use crate::types::{PageInfo, PayrixQuery, PayrixResponse};
use reqwest::{Client, Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{debug, warn};

/// Check if any errors indicate a rate limit exceeded condition.
fn is_rate_limit_error(errors: &[PayrixApiError]) -> bool {
    errors
        .iter()
        .any(|e| e.error_code.as_deref() == Some("C_RATE_LIMIT_EXCEEDED_TEMP_BLOCK"))
}

/// Payrix API environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Environment {
    /// Test/sandbox environment
    #[default]
    Test,
    /// Production environment
    Production,
}

impl Environment {
    fn base_url(&self) -> &'static str {
        match self {
            Environment::Test => "https://test-api.payrix.com/",
            Environment::Production => "https://api.payrix.com/",
        }
    }
}

/// Configuration for the Payrix client.
#[derive(Debug, Clone)]
pub struct Config {
    /// API key for authentication
    pub api_key: String,
    /// Environment (test or production)
    pub environment: Environment,
    /// Maximum retries for rate-limited requests
    pub max_retries: u32,
    /// Delay between retries (default: 10 seconds per Payrix docs)
    pub retry_delay: Duration,
    /// Custom base URL (overrides environment URL if set).
    ///
    /// This is primarily useful for testing with mock servers.
    /// If `None`, the URL is determined by the `environment` field.
    pub base_url: Option<String>,
}

impl Config {
    /// Create a new configuration.
    pub fn new(api_key: impl Into<String>, environment: Environment) -> Self {
        Self {
            api_key: api_key.into(),
            environment,
            max_retries: 3,
            retry_delay: Duration::from_secs(10),
            base_url: None,
        }
    }

    /// Set a custom base URL (for testing with mock servers).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{Config, Environment};
    ///
    /// let config = Config::new("api-key", Environment::Test)
    ///     .with_base_url("http://localhost:8080/");
    /// ```
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }
}

/// Payrix API client.
///
/// Handles authentication, rate limiting, and automatic retries.
#[derive(Debug, Clone)]
pub struct PayrixClient {
    http: Client,
    config: Config,
    rate_limiter: Arc<Mutex<RateLimiter>>,
}

impl PayrixClient {
    /// Create a new Payrix client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Payrix API key
    /// * `environment` - Test or Production environment
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment};
    ///
    /// let client = PayrixClient::new("your-api-key", Environment::Test).unwrap();
    /// ```
    pub fn new(api_key: impl Into<String>, environment: Environment) -> Result<Self> {
        let config = Config::new(api_key, environment);
        Self::with_config(config)
    }

    /// Create a client with custom configuration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Config, Environment};
    /// use std::time::Duration;
    ///
    /// let mut config = Config::new("api-key", Environment::Production);
    /// config.max_retries = 5;
    /// config.retry_delay = Duration::from_secs(15);
    ///
    /// let client = PayrixClient::with_config(config)?;
    /// # Ok::<(), payrix::Error>(())
    /// ```
    pub fn with_config(config: Config) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(Error::Config("API key cannot be empty".into()));
        }

        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(Error::Http)?;

        Ok(Self {
            http,
            config,
            rate_limiter: Arc::new(Mutex::new(RateLimiter::default_payrix())),
        })
    }

    /// Get the base URL for the configured environment.
    ///
    /// If a custom base URL is configured, it takes precedence over
    /// the environment's default URL.
    pub fn base_url(&self) -> &str {
        self.config
            .base_url
            .as_deref()
            .unwrap_or_else(|| self.config.environment.base_url())
    }

    /// Execute an HTTP request with rate limiting and retry logic.
    ///
    /// This is the core method that all API calls go through.
    async fn execute_with_retry<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&impl Serialize>,
    ) -> Result<Option<T>> {
        let url = format!("{}{}", self.base_url(), path);
        let mut retries = 0;

        loop {
            // 1. Check rate limit and wait if necessary
            let wait = self.rate_limiter.lock().await.check();
            if !wait.is_zero() {
                debug!(wait_ms = wait.as_millis(), "Rate limit: waiting before request");
                tokio::time::sleep(wait).await;
            }

            // 2. Build and send request
            let mut request = self.http.request(method.clone(), &url);
            request = request
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .header("APIKEY", &self.config.api_key);

            if let Some(b) = body {
                request = request.json(b);
            }

            debug!(method = %method, url = %url, "Sending Payrix request");
            let response = request.send().await?;
            let status = response.status();

            // 3. Handle HTTP-level errors
            if status == StatusCode::TOO_MANY_REQUESTS {
                if retries >= self.config.max_retries {
                    return Err(Error::RateLimited(
                        "Max retries exceeded for rate limiting".into(),
                    ));
                }
                retries += 1;
                warn!(retries, "Rate limited by Payrix, retrying after delay");
                tokio::time::sleep(self.config.retry_delay).await;
                continue;
            }

            if status == StatusCode::UNAUTHORIZED {
                return Err(Error::Unauthorized("Invalid API key".into()));
            }

            if status == StatusCode::NOT_FOUND {
                return Err(Error::NotFound(format!("Resource not found: {}", path)));
            }

            if status == StatusCode::SERVICE_UNAVAILABLE {
                return Err(Error::ServiceUnavailable(
                    "Payrix service is temporarily unavailable".into(),
                ));
            }

            if status == StatusCode::BAD_REQUEST {
                let text = response.text().await.unwrap_or_default();
                return Err(Error::BadRequest(text));
            }

            // 4. Parse JSON response
            let query: PayrixQuery<T> = response.json().await?;

            // 5. Handle Payrix's "200 with errors in body" pattern
            if !query.errors.is_empty() {
                // Check if it's a rate limit error in disguise
                if is_rate_limit_error(&query.errors) {
                    if retries >= self.config.max_retries {
                        return Err(Error::RateLimited(
                            "Max retries exceeded for rate limiting".into(),
                        ));
                    }
                    retries += 1;
                    warn!(retries, "Rate limited (in body), retrying after delay");
                    tokio::time::sleep(self.config.retry_delay).await;
                    continue;
                }

                return Err(Error::from_api_errors(query.errors));
            }

            // 6. Extract response data
            return match query.response {
                Some(resp) => {
                    if !resp.errors.is_empty() {
                        Err(Error::from_api_errors(resp.errors))
                    } else {
                        // Return first item if data exists
                        Ok(resp.data.into_iter().next())
                    }
                }
                None => Ok(None),
            };
        }
    }

    /// Execute a GET request.
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<Option<T>> {
        self.execute_with_retry::<T>(Method::GET, path, None::<&()>)
            .await
    }

    /// Execute a POST request.
    pub(crate) async fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &impl Serialize,
    ) -> Result<Option<T>> {
        self.execute_with_retry(Method::POST, path, Some(body))
            .await
    }

    /// Execute a PUT request.
    pub(crate) async fn put<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &impl Serialize,
    ) -> Result<Option<T>> {
        self.execute_with_retry(Method::PUT, path, Some(body)).await
    }

    /// Execute a DELETE request.
    pub(crate) async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<Option<T>> {
        self.execute_with_retry::<T>(Method::DELETE, path, None::<&()>)
            .await
    }

    // ========================================================================
    // Public API Methods
    // ========================================================================

    /// Create a new entity.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment, EntityType, CreateCustomer};
    ///
    /// # async fn example() -> payrix::Result<()> {
    /// let client = PayrixClient::new("api-key", Environment::Test)?;
    ///
    /// let customer = client.create::<_, payrix::Customer>(
    ///     EntityType::Customers,
    ///     &CreateCustomer {
    ///         merchant: Some("t1_mer_12345678901234567890123".parse().unwrap()),
    ///         first: Some("John".to_string()),
    ///         last: Some("Doe".to_string()),
    ///         ..Default::default()
    ///     }
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create<B: Serialize, T: DeserializeOwned>(
        &self,
        entity_type: EntityType,
        body: &B,
    ) -> Result<T> {
        self.post::<T>(entity_type.as_str(), body)
            .await?
            .ok_or_else(|| Error::Internal("No response from create".into()))
    }

    /// Update an existing entity.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment, EntityType};
    /// use serde_json::json;
    ///
    /// # async fn example() -> payrix::Result<()> {
    /// let client = PayrixClient::new("api-key", Environment::Test)?;
    ///
    /// let updated: payrix::Customer = client.update(
    ///     EntityType::Customers,
    ///     "customer_id",
    ///     &json!({ "first": "Jane" })
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update<B: Serialize, T: DeserializeOwned>(
        &self,
        entity_type: EntityType,
        id: &str,
        body: &B,
    ) -> Result<T> {
        let path = format!("{}/{}", entity_type.as_str(), id);
        self.put::<T>(&path, body)
            .await?
            .ok_or_else(|| Error::Internal("No response from update".into()))
    }

    /// Delete an entity by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment, EntityType};
    ///
    /// # async fn example() -> payrix::Result<()> {
    /// let client = PayrixClient::new("api-key", Environment::Test)?;
    ///
    /// client.remove::<payrix::Token>(EntityType::Tokens, "token_id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove<T: DeserializeOwned>(&self, entity_type: EntityType, id: &str) -> Result<T> {
        let path = format!("{}/{}", entity_type.as_str(), id);
        self.delete::<T>(&path)
            .await?
            .ok_or_else(|| Error::Internal("No response from delete".into()))
    }

    /// Get a single entity by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment, EntityType, Customer};
    ///
    /// # async fn example() -> payrix::Result<()> {
    /// let client = PayrixClient::new("api-key", Environment::Test)?;
    ///
    /// let customer: Option<Customer> = client.get_one(
    ///     EntityType::Customers,
    ///     "customer_id"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_one<T: DeserializeOwned>(
        &self,
        entity_type: EntityType,
        id: &str,
    ) -> Result<Option<T>> {
        let path = format!("{}/{}", entity_type.as_str(), id);
        self.get(&path).await
    }

    /// Get a single entity by ID with expanded relations.
    ///
    /// # Arguments
    ///
    /// * `entity_type` - The type of entity to fetch
    /// * `id` - The entity ID
    /// * `expand` - Relations to expand (e.g., `["token", "customer"]`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment, EntityType, Transaction};
    ///
    /// # async fn example() -> payrix::Result<()> {
    /// let client = PayrixClient::new("api-key", Environment::Test)?;
    ///
    /// // Get a transaction with its token and customer expanded
    /// let txn: Option<Transaction> = client.get_one_expanded(
    ///     EntityType::Txns,
    ///     "txn_id",
    ///     &["token", "customer"]
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_one_expanded<T: DeserializeOwned>(
        &self,
        entity_type: EntityType,
        id: &str,
        expand: &[&str],
    ) -> Result<Option<T>> {
        let expand_query = build_expand_query(expand);
        let path = format!("{}/{}?{}", entity_type.as_str(), id, expand_query);
        self.get(&path).await
    }

    /// Get all entities of a type with pagination.
    ///
    /// This method automatically handles pagination and returns all matching entities.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment, EntityType, Customer};
    ///
    /// # async fn example() -> payrix::Result<()> {
    /// let client = PayrixClient::new("api-key", Environment::Test)?;
    ///
    /// let customers: Vec<Customer> = client.get_all(EntityType::Customers).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_all<T: DeserializeOwned>(&self, entity_type: EntityType) -> Result<Vec<T>> {
        self.get_all_with_params::<T>(entity_type, HashMap::new(), None).await
    }

    /// Get all entities with custom parameters and optional search.
    ///
    /// # Arguments
    ///
    /// * `entity_type` - The type of entity to fetch
    /// * `params` - Query parameters (e.g., filters)
    /// * `search` - Optional search string
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment, EntityType, Transaction};
    /// use std::collections::HashMap;
    ///
    /// # async fn example() -> payrix::Result<()> {
    /// let client = PayrixClient::new("api-key", Environment::Test)?;
    ///
    /// // Get transactions with custom sorting
    /// let mut params = HashMap::new();
    /// params.insert("sort".to_string(), "created[desc]".to_string());
    ///
    /// let txns: Vec<Transaction> = client.get_all_with_params(
    ///     EntityType::Txns,
    ///     params,
    ///     Some("status[equals]=1")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_all_with_params<T: DeserializeOwned>(
        &self,
        entity_type: EntityType,
        params: HashMap<String, String>,
        search: Option<&str>,
    ) -> Result<Vec<T>> {
        let mut items = Vec::new();
        let mut page = 1;
        let page_limit = 100; // Payrix max

        loop {
            let (data, page_info) = self
                .get_page::<T>(entity_type, page, page_limit, &params, search)
                .await?;

            items.extend(data);

            if !page_info.has_more {
                break;
            }
            page += 1;
        }

        Ok(items)
    }

    /// Get a single page of entities.
    ///
    /// # Returns
    ///
    /// A tuple of (data, page_info) where page_info contains pagination details.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment, EntityType, Transaction};
    /// use std::collections::HashMap;
    ///
    /// # async fn example() -> payrix::Result<()> {
    /// let client = PayrixClient::new("api-key", Environment::Test)?;
    ///
    /// // Get the first page of 50 transactions
    /// let (transactions, page_info) = client.get_page::<Transaction>(
    ///     EntityType::Txns,
    ///     1,      // page number
    ///     50,     // page limit
    ///     &HashMap::new(),
    ///     None
    /// ).await?;
    ///
    /// println!("Got {} transactions, has_more: {}", transactions.len(), page_info.has_more);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_page<T: DeserializeOwned>(
        &self,
        entity_type: EntityType,
        page: i32,
        limit: i32,
        params: &HashMap<String, String>,
        search: Option<&str>,
    ) -> Result<(Vec<T>, PageInfo)> {
        let mut query_parts: Vec<String> = vec![
            format!("page[number]={}", page),
            format!("page[limit]={}", limit),
        ];

        for (key, value) in params {
            query_parts.push(format!("{}={}", key, value));
        }

        let path = format!("{}?{}", entity_type.as_str(), query_parts.join("&"));
        let response = self.get_raw::<T>(&path, search).await?;

        match response {
            Some(resp) => Ok((resp.data, resp.details.page)),
            None => Ok((Vec::new(), PageInfo::default())),
        }
    }

    /// Search for entities matching a query.
    ///
    /// # Arguments
    ///
    /// * `entity_type` - The type of entity to search
    /// * `search` - Search query string (Payrix search syntax)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment, EntityType, Token};
    ///
    /// # async fn example() -> payrix::Result<()> {
    /// let client = PayrixClient::new("api-key", Environment::Test)?;
    ///
    /// // Find tokens for a specific customer
    /// let tokens: Vec<Token> = client.search(
    ///     EntityType::Tokens,
    ///     "customer[equals]=cust_123"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search<T: DeserializeOwned>(
        &self,
        entity_type: EntityType,
        search: &str,
    ) -> Result<Vec<T>> {
        self.get_all_with_params(entity_type, HashMap::new(), Some(search))
            .await
    }

    /// Find a single entity matching a search query.
    ///
    /// Returns the first match, or None if no matches found.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment, EntityType, Customer};
    ///
    /// # async fn example() -> payrix::Result<()> {
    /// let client = PayrixClient::new("api-key", Environment::Test)?;
    ///
    /// // Find a customer by email
    /// let customer: Option<Customer> = client.find_one(
    ///     EntityType::Customers,
    ///     "email[equals]=john@example.com"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find_one<T: DeserializeOwned>(
        &self,
        entity_type: EntityType,
        search: &str,
    ) -> Result<Option<T>> {
        let results: Vec<T> = self.search(entity_type, search).await?;
        Ok(results.into_iter().next())
    }

    /// Execute a GET request and return the full response.
    async fn get_raw<T: DeserializeOwned>(
        &self,
        path: &str,
        search: Option<&str>,
    ) -> Result<Option<PayrixResponse<T>>> {
        let url = format!("{}{}", self.base_url(), path);
        let mut retries = 0;

        loop {
            let wait = self.rate_limiter.lock().await.check();
            if !wait.is_zero() {
                tokio::time::sleep(wait).await;
            }

            let mut request = self.http.get(&url);
            request = request
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .header("APIKEY", &self.config.api_key);

            if let Some(s) = search {
                request = request.header("search", s);
            }

            let response = request.send().await?;
            let status = response.status();

            if status == StatusCode::TOO_MANY_REQUESTS {
                if retries >= self.config.max_retries {
                    return Err(Error::RateLimited("Max retries exceeded".into()));
                }
                retries += 1;
                tokio::time::sleep(self.config.retry_delay).await;
                continue;
            }

            if status == StatusCode::UNAUTHORIZED {
                return Err(Error::Unauthorized("Invalid API key".into()));
            }

            let query: PayrixQuery<T> = response.json().await?;

            if !query.errors.is_empty() {
                if is_rate_limit_error(&query.errors) {
                    if retries >= self.config.max_retries {
                        return Err(Error::RateLimited("Max retries exceeded".into()));
                    }
                    retries += 1;
                    tokio::time::sleep(self.config.retry_delay).await;
                    continue;
                }

                return Err(Error::from_api_errors(query.errors));
            }

            return Ok(query.response);
        }
    }
}
