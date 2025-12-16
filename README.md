# payrix

A Rust client library for the [Payrix](https://www.payrix.com/) payment processing API.

[![Crates.io](https://img.shields.io/crates/v/payrix.svg)](https://crates.io/crates/payrix)
[![Documentation](https://docs.rs/payrix/badge.svg)](https://docs.rs/payrix)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Full async/await support** with Tokio
- **Built-in rate limiting** to avoid API throttling
- **Automatic retry** with exponential backoff for transient failures
- **Strongly typed** API responses with 68 enums and 26 resource types
- **Comprehensive error handling** with domain-specific error types
- **Optional SQLx support** for database integration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
payrix = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

### Optional Features

```toml
# Enable SQLx derive macros for database storage
payrix = { version = "0.1", features = ["sqlx"] }
```

## Quick Start

```rust,no_run
use payrix::{PayrixClient, Environment, EntityType, Customer, NewCustomer};

#[tokio::main]
async fn main() -> Result<(), payrix::Error> {
    // Create a client
    let client = PayrixClient::new("your-api-key", Environment::Test)?;

    // Get a customer by ID
    let customer: Option<Customer> = client.get_one(
        EntityType::Customers,
        "t1_cus_12345678901234567890123"
    ).await?;

    // Create a new customer
    let new_customer: Customer = client.create(
        EntityType::Customers,
        &NewCustomer {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            first: Some("John".to_string()),
            last: Some("Doe".to_string()),
            email: Some("john@example.com".to_string()),
            ..Default::default()
        }
    ).await?;

    Ok(())
}
```

## Common Operations

### Creating a Transaction

```rust,no_run
use payrix::{PayrixClient, Environment, EntityType, Transaction, NewTransaction};

async fn charge_card(client: &PayrixClient) -> Result<Transaction, payrix::Error> {
    client.create(
        EntityType::Txns,
        &NewTransaction {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            token: Some("t1_tok_12345678901234567890123".to_string()),
            total: 1000, // $10.00 in cents
            ..Default::default()
        }
    ).await
}
```

### Searching for Records

```rust,no_run
use payrix::{PayrixClient, EntityType, Token, SearchBuilder, SearchOperator};

async fn find_customer_tokens(
    client: &PayrixClient,
    customer_id: &str
) -> Result<Vec<Token>, payrix::Error> {
    // Using SearchBuilder
    let search = SearchBuilder::new()
        .field("customer", customer_id)
        .build();

    client.search(EntityType::Tokens, &search).await
}
```

### Pagination

```rust,no_run
use payrix::{PayrixClient, EntityType, Transaction};

async fn get_all_transactions(client: &PayrixClient) -> Result<Vec<Transaction>, payrix::Error> {
    // Automatically handles pagination
    client.get_all(EntityType::Txns).await
}
```

### Expanding Related Resources

```rust,no_run
use payrix::{PayrixClient, EntityType, Transaction};

async fn get_transaction_with_token(
    client: &PayrixClient,
    txn_id: &str
) -> Result<Option<Transaction>, payrix::Error> {
    // Expand the token relation
    client.get_one_expanded(
        EntityType::Txns,
        txn_id,
        &["token", "customer"]
    ).await
}
```

## API Coverage

Based on the [Payrix API documentation](https://resource.payrix.com/resources/api):

| Category | Resources |
|----------|-----------|
| **Core** | Merchants, Entities, Customers, Tokens, Transactions |
| **Banking** | Accounts, AccountVerifications, Funds, Disbursements |
| **Billing** | Subscriptions, Plans, Fees, FeeRules |
| **Operations** | Batches, Payouts, Reserves, Adjustments |
| **Disputes** | Chargebacks, ChargebackMessages, ChargebackDocuments |
| **Admin** | Orgs, TeamLogins, Members, Contacts, Vendors |

## Environment Configuration

```rust,no_run
use payrix::{PayrixClient, Environment};

// Test environment (sandbox)
let test_client = PayrixClient::new("test-api-key", Environment::Test)?;

// Production environment
let prod_client = PayrixClient::new("prod-api-key", Environment::Production)?;
```

| Environment | Base URL |
|-------------|----------|
| Test | `https://test-api.payrix.com/` |
| Production | `https://api.payrix.com/` |

## Error Handling

```rust,no_run
use payrix::{PayrixClient, Environment, EntityType, Customer, Error};

async fn handle_errors(client: &PayrixClient) {
    let result: Result<Option<Customer>, Error> = client.get_one(
        EntityType::Customers,
        "invalid_id"
    ).await;

    match result {
        Ok(Some(customer)) => println!("Found: {:?}", customer),
        Ok(None) => println!("Customer not found"),
        Err(Error::Unauthorized(_)) => println!("Invalid API key"),
        Err(Error::RateLimited(_)) => println!("Rate limited, retry later"),
        Err(Error::Api(errors)) => println!("API errors: {:?}", errors),
        Err(e) => println!("Other error: {}", e),
    }
}
```

## Design Decisions

### Rate Limiting Strategy

The client implements two-layer rate limiting:

1. **Proactive** - Tracks outgoing requests with a sliding window to stay under API limits
2. **Reactive** - Handles 429 responses with exponential backoff (10s sleep, up to 3 retries)

### Why Not Tower Middleware?

This library uses method-specific helpers instead of Tower middleware:

- **Simplicity** - Tower's trait bounds add complexity without proportional benefit for a single API client
- **Payrix quirks** - Payrix returns HTTP 200 with errors in the JSON body, requiring custom parsing
- **Debuggability** - Simple loops are easier to trace than middleware stacks
- **Code size** - ~50 lines for retry+rate limiting vs 100+ for Tower setup

### Async-First

All API methods are async and require a Tokio runtime. This matches the reality of HTTP API clients and enables efficient concurrent requests.

## API Reference

- [Payrix API Documentation](https://resource.payrix.com/resources/api)
- [API Call Syntax](https://resource.payrix.com/resources/api-call-syntax)
- [Search Operators](https://resource.payrix.com/resources/api-search-operators)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This is an unofficial client library. Payrix and Worldpay are trademarks of their respective owners.
