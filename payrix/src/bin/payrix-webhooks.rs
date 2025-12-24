//! CLI binary for Payrix webhook server management.
//!
//! This binary provides commands to:
//! - Run a webhook server to receive Payrix events
//! - Set up webhook alerts in Payrix
//! - Check existing webhook configuration
//! - Remove webhook alerts
//!
//! # Usage
//!
//! ```bash
//! # Run the webhook server on port 13847 (default)
//! payrix-webhooks serve
//!
//! # Run on a custom port with auth header
//! payrix-webhooks serve --port 8080 --auth-header X-Secret --auth-value my-secret
//!
//! # Set up webhook alerts in Payrix
//! payrix-webhooks setup --base-url https://api.example.com --events chargeback
//!
//! # Check current webhook configuration
//! payrix-webhooks status
//!
//! # Remove all webhook alerts
//! payrix-webhooks remove
//! ```
//!
//! # Environment Variables
//!
//! - `TEST_PAYRIX_API_KEY` - Your Payrix API key (required for setup/status/remove commands)

use std::net::SocketAddr;

use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use payrix::{
    webhooks::{WebhookServer, WebhookServerConfig},
    workflows::webhook_setup::{
        get_webhook_status, remove_webhooks, setup_webhooks, WebhookConfig, WebhookEventType,
    },
    Environment, PayrixClient,
};

#[derive(Parser)]
#[command(name = "payrix-webhooks")]
#[command(about = "Payrix webhook server and management CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the webhook server to receive Payrix events
    Serve {
        /// Port to listen on (default: 13847)
        #[arg(short, long, default_value = "13847")]
        port: u16,

        /// Bind address (default: 0.0.0.0)
        #[arg(short, long, default_value = "0.0.0.0")]
        bind: String,

        /// Authentication header name (e.g., "X-Webhook-Secret")
        #[arg(long)]
        auth_header: Option<String>,

        /// Authentication header value
        #[arg(long)]
        auth_value: Option<String>,

        /// Allowed IP CIDRs (can specify multiple)
        #[arg(long)]
        allow_ip: Vec<String>,

        /// Disable stdout logging
        #[arg(long)]
        quiet: bool,
    },

    /// Set up webhook alerts in Payrix
    Setup {
        /// Base URL for your webhook endpoint (e.g., "https://api.example.com")
        #[arg(long)]
        base_url: String,

        /// Webhook path (default: /webhooks/payrix)
        #[arg(long, default_value = "/webhooks/payrix")]
        path: String,

        /// Event types to subscribe to (chargeback, transaction, merchant, disbursement, all)
        #[arg(long, value_delimiter = ',')]
        events: Vec<String>,

        /// Authentication header name
        #[arg(long)]
        auth_header: Option<String>,

        /// Authentication header value
        #[arg(long)]
        auth_value: Option<String>,

        /// Alert name
        #[arg(long)]
        name: Option<String>,

        /// Use test environment
        #[arg(long)]
        test: bool,
    },

    /// Check current webhook configuration
    Status {
        /// Use test environment
        #[arg(long)]
        test: bool,
    },

    /// Remove all webhook alerts
    Remove {
        /// Use test environment
        #[arg(long)]
        test: bool,

        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Serve {
            port,
            bind,
            auth_header,
            auth_value,
            allow_ip,
            quiet,
        } => {
            run_server(port, bind, auth_header, auth_value, allow_ip, quiet).await?;
        }
        Commands::Setup {
            base_url,
            path,
            events,
            auth_header,
            auth_value,
            name,
            test,
        } => {
            run_setup(base_url, path, events, auth_header, auth_value, name, test).await?;
        }
        Commands::Status { test } => {
            run_status(test).await?;
        }
        Commands::Remove { test, yes } => {
            run_remove(test, yes).await?;
        }
    }

    Ok(())
}

async fn run_server(
    port: u16,
    bind: String,
    auth_header: Option<String>,
    auth_value: Option<String>,
    allow_ip: Vec<String>,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse allowed IPs
    let allowed_ips: Vec<ipnet::IpNet> = allow_ip
        .iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?;

    // Build config
    let mut config = WebhookServerConfig::new()
        .with_stdout_logging(!quiet);

    if !allowed_ips.is_empty() {
        config = config.with_allowed_ips(allowed_ips);
    }

    if let (Some(name), Some(value)) = (auth_header, auth_value) {
        config = config.with_auth_header(name, value);
    }

    // Create server
    let (server, mut events) = WebhookServer::with_config(config);

    // Spawn event handler
    tokio::spawn(async move {
        while let Some(event) = events.recv().await {
            // Check for chargeback events
            if let Some(cb_event) = event.as_chargeback_event() {
                let event_name = match &cb_event {
                    payrix::webhooks::ChargebackEvent::Created { .. } => "created",
                    payrix::webhooks::ChargebackEvent::Opened { .. } => "opened",
                    payrix::webhooks::ChargebackEvent::Closed { .. } => "closed",
                    payrix::webhooks::ChargebackEvent::Won { .. } => "won",
                    payrix::webhooks::ChargebackEvent::Lost { .. } => "lost",
                    payrix::webhooks::ChargebackEvent::Other { .. } => "other",
                };
                tracing::info!(
                    chargeback_id = %cb_event.chargeback_id(),
                    event_type = %event_name,
                    "Received chargeback event"
                );
            }
        }
    });

    // Run server
    let addr: SocketAddr = format!("{}:{}", bind, port).parse()?;
    println!("Starting webhook server on {}", addr);
    println!("Webhook endpoint: POST {}/webhooks/payrix", addr);
    println!("Health check:     GET  {}/health", addr);
    println!();
    println!("Press Ctrl+C to stop");

    server.run(addr).await?;

    Ok(())
}

async fn run_setup(
    base_url: String,
    path: String,
    events: Vec<String>,
    auth_header: Option<String>,
    auth_value: Option<String>,
    name: Option<String>,
    test: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client(test)?;

    // Parse events
    let event_types = parse_event_types(&events)?;
    if event_types.is_empty() {
        eprintln!("Error: No events specified. Use --events with one of: chargeback, transaction, merchant, disbursement, all");
        std::process::exit(1);
    }

    // Build config
    let mut config = WebhookConfig::new(base_url)
        .with_path(path)
        .with_events(event_types);

    if let (Some(h), Some(v)) = (auth_header, auth_value) {
        config = config.with_auth(h, v);
    }

    if let Some(n) = name {
        config = config.with_name(n);
    }

    println!("Setting up webhooks...");
    println!("  Endpoint: {}", config.webhook_url());
    println!("  Events: {:?}", config.events.iter().map(|e| e.as_event_str()).collect::<Vec<_>>());

    let result = setup_webhooks(&client, config).await?;

    println!();
    println!("Webhook setup complete!");
    println!("  Alert ID: {}", result.alert_id);
    println!("  Action ID: {}", result.action_id);
    println!("  Triggers created: {}", result.triggers_created.len());
    for trigger in &result.triggers_created {
        println!("    - {}", trigger);
    }

    Ok(())
}

async fn run_status(test: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client(test)?;

    println!("Checking webhook configuration...");
    let status = get_webhook_status(&client).await?;

    if status.alerts.is_empty() {
        println!();
        println!("No webhook alerts configured.");
        return Ok(());
    }

    println!();
    println!("Configured webhooks:");
    for alert in &status.alerts {
        println!();
        println!("  Alert: {} ({})", alert.name, alert.id);
        println!("  Endpoint: {}", alert.endpoint);
        println!("  Active: {}", if alert.is_active { "Yes" } else { "No" });
        if let Some(ref auth) = alert.auth_header {
            println!("  Auth Header: {}", auth);
        }
        println!("  Events:");
        for event in &alert.events {
            println!("    - {}", event);
        }
    }

    Ok(())
}

async fn run_remove(test: bool, yes: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client(test)?;

    // First check what we're going to remove
    let status = get_webhook_status(&client).await?;

    if status.alerts.is_empty() {
        println!("No webhook alerts to remove.");
        return Ok(());
    }

    println!("Found {} webhook alert(s) to remove:", status.alerts.len());
    for alert in &status.alerts {
        println!("  - {} ({}) -> {}", alert.name, alert.id, alert.endpoint);
    }

    if !yes {
        println!();
        println!("Are you sure you want to remove all webhook alerts? (y/N)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    println!();
    println!("Removing webhooks...");
    let count = remove_webhooks(&client).await?;
    println!("Removed {} webhook alert(s).", count);

    Ok(())
}

fn get_client(test: bool) -> Result<PayrixClient, Box<dyn std::error::Error>> {
    let api_key = std::env::var("TEST_PAYRIX_API_KEY")
        .map_err(|_| "TEST_PAYRIX_API_KEY environment variable not set")?;

    let env = if test {
        Environment::Test
    } else {
        Environment::Production
    };

    let client = PayrixClient::new(&api_key, env)?;
    Ok(client)
}

fn parse_event_types(events: &[String]) -> Result<Vec<WebhookEventType>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();

    for event in events {
        match event.to_lowercase().as_str() {
            "all" => {
                result.extend(WebhookEventType::all_chargeback_events());
                result.extend(WebhookEventType::all_transaction_events());
                result.extend(WebhookEventType::all_merchant_events());
                result.extend(WebhookEventType::all_disbursement_events());
            }
            "chargeback" | "chargebacks" => {
                result.extend(WebhookEventType::all_chargeback_events());
            }
            "transaction" | "transactions" | "txn" => {
                result.extend(WebhookEventType::all_transaction_events());
            }
            "merchant" | "merchants" => {
                result.extend(WebhookEventType::all_merchant_events());
            }
            "disbursement" | "disbursements" => {
                result.extend(WebhookEventType::all_disbursement_events());
            }
            // Allow specific event names
            "chargeback.created" => result.push(WebhookEventType::ChargebackCreated),
            "chargeback.opened" => result.push(WebhookEventType::ChargebackOpened),
            "chargeback.closed" => result.push(WebhookEventType::ChargebackClosed),
            "chargeback.won" => result.push(WebhookEventType::ChargebackWon),
            "chargeback.lost" => result.push(WebhookEventType::ChargebackLost),
            "txn.created" => result.push(WebhookEventType::TransactionCreated),
            "txn.approved" => result.push(WebhookEventType::TransactionApproved),
            "txn.failed" => result.push(WebhookEventType::TransactionFailed),
            "merchant.created" => result.push(WebhookEventType::MerchantCreated),
            "merchant.boarded" => result.push(WebhookEventType::MerchantBoarded),
            other => {
                eprintln!("Warning: Unknown event type '{}', skipping", other);
            }
        }
    }

    // Deduplicate
    result.sort_by_key(|e| e.as_event_str());
    result.dedup_by_key(|e| e.as_event_str());

    Ok(result)
}
