# payrix

A Rust client library for the [Payrix](https://www.payrix.com/) payment processing API.

This library was created based on our experience implementing Payrix Pro at our company,
18 months of experience with the quirks and peculiarities of the interface. This was
also *not* our first payment provider integration.

We used Claude Code's Opus 4.5 thinking mode to help with grunt work, like taking
the OpenAPI spec and ensuring that we had all of the Enums correct, and documenting
return codes (first quirk: errors are returned with an HTTP status code 200, plus
an array of errors, in one of two places in the result, depending on the kind of 
error).

We provide a layered set of API calls, starting with low-level calls directly
to each Payrix endpoint (e.g. /merchant), then layering workflow-specific functions
to keep the code idiomatic and encapsulate best practices.

Author: john@outlawpractice.com

[![Crates.io](https://img.shields.io/crates/v/payrix.svg)](https://crates.io/crates/payrix)
[![Documentation](https://docs.rs/payrix/badge.svg)](https://docs.rs/payrix)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Full async/await support** with Tokio
- **Built-in rate limiting** to avoid API throttling
- **Automatic retry** with exponential backoff for transient failures
- **Strongly typed** API responses with 68 enums and 80 struct types
- **Comprehensive error handling** with domain-specific error types
- **Optional SQLx support** for database integration
- **OpenAPI 3.1 spec** included for reference (`openApi/payrix-openapi31.yaml`)
- **API discrepancy documentation** cataloging differences between docs and reality

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
use payrix::{PayrixClient, Environment, EntityType, Customer};

#[tokio::main]
async fn main() -> Result<(), payrix::Error> {
    // Create a client
    let client = PayrixClient::new("your-api-key", Environment::Test)?;

    // Get a customer by ID
    let customer: Option<Customer> = client.get_one(
        EntityType::Customers,
        "t1_cus_12345678901234567890123"
    ).await?;

    Ok(())
}
```

## Common Operations

### Creating a Transaction

```rust,no_run
use payrix::{PayrixClient, Environment, EntityType, Transaction};

async fn charge_card(client: &PayrixClient) -> Result<Transaction, payrix::Error> {
    client.create(
        EntityType::Txns,
        &serde_json::json!({
            "merchant": "t1_mer_12345678901234567890123",
            "token": "t1_tok_12345678901234567890123",
            "total": 1000  // $10.00 in cents
        })
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

## High-Level Workflows

The library includes workflow modules that encapsulate complex multi-step operations:

### Merchant Onboarding

```rust,no_run
use payrix::{PayrixClient, Environment, onboard_merchant, OnboardMerchantRequest};
use payrix::{BusinessInfo, MerchantConfig, BankAccountInfo, MemberInfo, Address, TermsAcceptance};
use payrix::types::{MerchantType, MemberType, AccountHolderType, MerchantEnvironment, DateYmd};

async fn onboard_new_merchant(client: &PayrixClient) -> Result<(), payrix::Error> {
    let result = onboard_merchant(client, OnboardMerchantRequest {
        business: BusinessInfo {
            business_type: MerchantType::Llc,
            legal_name: "Acme Corp LLC".to_string(),
            address: Address {
                line1: "123 Main St".to_string(),
                line2: None,
                city: "Chicago".to_string(),
                state: "IL".to_string(),
                zip: "60601".to_string(),
                country: "USA".to_string(),
            },
            phone: "3125551234".to_string(),
            email: "contact@acme.com".to_string(),
            website: Some("https://acme.com".to_string()),
            ein: "123456789".to_string(),
        },
        merchant: MerchantConfig {
            dba: "Acme Services".to_string(),
            mcc: "5812".to_string(),
            environment: MerchantEnvironment::Ecommerce,
            annual_cc_sales: 500_000_00,  // $500,000 in cents
            avg_ticket: 50_00,             // $50 in cents
            established: DateYmd::new("20200101").unwrap(),
            is_new_business: false,
        },
        accounts: vec![BankAccountInfo {
            routing_number: "071000013".to_string(),
            account_number: "123456789".to_string(),
            account_type: AccountHolderType::Business,
            is_primary: true,
        }],
        members: vec![MemberInfo {
            member_type: MemberType::Owner,
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            title: Some("CEO".to_string()),
            ownership_percentage: 100,
            date_of_birth: DateYmd::new("19800115").unwrap(),
            ssn: "123456789".to_string(),
            email: "jane@acme.com".to_string(),
            phone: "3125559876".to_string(),
            address: Address {
                line1: "456 Oak Ave".to_string(),
                line2: None,
                city: "Chicago".to_string(),
                state: "IL".to_string(),
                zip: "60602".to_string(),
                country: "USA".to_string(),
            },
        }],
        terms_acceptance: TermsAcceptance {
            version: "4.21".to_string(),
            accepted_at: "2024-01-15 10:30:00".to_string(),
        },
    }).await?;

    println!("Merchant {} created with status {:?}",
             result.merchant_id, result.boarding_status);
    Ok(())
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

### Reality Wins

When the OpenAPI spec differs from actual API behavior, we follow reality:

- **Flexible deserializers** - Transaction enums accept both integer and string formats (the API returns both depending on context)
- **Undocumented variants** - Added enum values observed in production that aren't in OpenAPI
- **Optional fields** - Made fields like `Customer.merchant` optional when the API returns null
- **Integer vs string enums** - `FeeType`, `FeeUnit`, `FeeCollection` use integers despite OpenAPI suggesting strings

See [API_INCONSISTENCIES.md](API_INCONSISTENCIES.md) for the full catalog of discrepancies.

## Test Results

| Category | Count | Status |
|----------|-------|--------|
| Unit tests | 586 | All passing |
| Doc tests | 27 | All passing |
| Integration tests | 52 | Ignored (require API key) |

Unit tests verify serialization, type conversions, and workflow payload structure without requiring API access.

Run unit and doc tests:

```bash
cargo test
```

Run integration tests (requires API credentials):

```bash
PAYRIX_API_KEY=your_key cargo test -- --ignored
```

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
