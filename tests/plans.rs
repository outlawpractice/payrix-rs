//! Plan and Subscription integration tests.

mod common;

use common::{create_client, init_logging, test_merchant_id};
use payrix::{
    CreateCustomer, CreateToken, Customer, EntityType, Environment, PaymentInfo, PaymentMethod,
    PayrixClient, Plan, PlanExpanded, Subscription, SubscriptionExpanded, Token,
};
use serde_json::{json, Value};
use std::env;

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_plans() {
    init_logging();
    let client = create_client();

    let plans: Vec<Plan> = client.get_all(EntityType::Plans).await.unwrap();

    println!("Found {} plans", plans.len());
    for plan in plans.iter().take(5) {
        println!(
            "  Plan: {} - name: {:?}, amount: {:?}",
            plan.id.as_str(),
            plan.name,
            plan.amount
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_subscriptions() {
    init_logging();
    let client = create_client();

    let subscriptions: Vec<Subscription> =
        client.get_all(EntityType::Subscriptions).await.unwrap();

    println!("Found {} subscriptions", subscriptions.len());
    for sub in subscriptions.iter().take(5) {
        println!(
            "  Subscription: {} - start: {:?}, plan: {:?}",
            sub.id.as_str(),
            sub.start,
            sub.plan
        );
    }
}

/// Test Plan CRUD operations.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources"]
async fn test_plan_crud() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== PLAN CRUD TEST ===\n");

    // CREATE
    let new_plan = json!({
        "merchant": test_merchant_id(),
        "type": "recurring",
        "name": format!("Test Plan {}", timestamp),
        "description": "Integration test plan",
        "schedule": 3,  // Monthly
        "scheduleFactor": 1,
        "um": "actual",
        "amount": 1999,  // $19.99
        "maxFailures": 3
    });

    let plan: Plan = client
        .create(EntityType::Plans, &new_plan)
        .await
        .expect("Failed to create plan");

    println!("Created plan: {}", plan.id.as_str());
    println!("  Name: {:?}", plan.name);
    println!("  Amount: {:?}", plan.amount);
    println!("  Schedule: {:?}", plan.schedule);

    assert!(plan.id.as_str().starts_with("t1_pln_"));
    assert_eq!(
        plan.name.as_deref(),
        Some(&format!("Test Plan {}", timestamp) as &str)
    );
    assert_eq!(plan.amount, Some(1999));

    // READ
    let fetched: Option<Plan> = client
        .get_one(EntityType::Plans, plan.id.as_str())
        .await
        .expect("Failed to get plan");

    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.id.as_str(), plan.id.as_str());
    println!("Read plan: {} - {:?}", fetched.id.as_str(), fetched.name);

    // UPDATE
    let updated: Plan = client
        .update(
            EntityType::Plans,
            plan.id.as_str(),
            &json!({"description": "Updated test plan description"}),
        )
        .await
        .expect("Failed to update plan");

    assert_eq!(
        updated.description.as_deref(),
        Some("Updated test plan description")
    );
    println!("Updated plan description: {:?}", updated.description);

    // DELETE (deactivate)
    let deactivated: Plan = client
        .update(EntityType::Plans, plan.id.as_str(), &json!({"inactive": 1}))
        .await
        .expect("Failed to deactivate plan");

    assert!(deactivated.inactive);
    println!("Deactivated plan: {}", deactivated.id.as_str());

    println!("\n=== PLAN CRUD TEST COMPLETE ===\n");
}

/// Test Subscription CRUD operations.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources"]
async fn test_subscription_crud() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== SUBSCRIPTION CRUD TEST ===\n");

    // First create prerequisites: customer, token, and plan
    let customer: Customer = client
        .create(
            EntityType::Customers,
            &CreateCustomer {
                merchant: Some(test_merchant_id().parse().unwrap()),
                first: Some(format!("SubTest{}", timestamp)),
                last: Some("Customer".to_string()),
                email: Some("payrixrust@gmail.com".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create customer");
    println!("Created customer: {}", customer.id.as_str());

    let token: Token = client
        .create(
            EntityType::Tokens,
            &CreateToken {
                customer: customer.id.to_string().parse().unwrap(),
                payment: PaymentInfo {
                    method: PaymentMethod::Visa,
                    number: Some("4111111111111111".to_string()),
                    routing: None,
                    expiration: Some("1230".to_string()),
                    cvv: Some("123".to_string()),
                },
                login: None,
                expiration: None,
                name: None,
                description: None,
                custom: None,
                inactive: None,
                frozen: None,
            },
        )
        .await
        .expect("Failed to create token");
    println!("Created token: {}", token.id.as_str());

    let plan: Plan = client
        .create(
            EntityType::Plans,
            &json!({
                "merchant": test_merchant_id(),
                "type": "recurring",
                "name": format!("Sub Test Plan {}", timestamp),
                "schedule": 3,
                "scheduleFactor": 1,
                "um": "actual",
                "amount": 999
            }),
        )
        .await
        .expect("Failed to create plan");
    println!("Created plan: {}", plan.id.as_str());

    // Calculate start date 30 days from now (YYYYMMDD format)
    let start_date = chrono::Utc::now() + chrono::Duration::days(30);
    let start_date_int = start_date
        .format("%Y%m%d")
        .to_string()
        .parse::<i32>()
        .unwrap();

    // CREATE subscription
    let new_sub = json!({
        "plan": plan.id.as_str(),
        "start": start_date_int,
        "origin": 2  // eCommerce
    });

    let subscription: Subscription = client
        .create(EntityType::Subscriptions, &new_sub)
        .await
        .expect("Failed to create subscription");

    println!("Created subscription: {}", subscription.id.as_str());
    println!("  Plan: {:?}", subscription.plan);
    println!("  Start: {:?}", subscription.start);

    assert!(subscription.id.as_str().starts_with("t1_sbn_"));

    // READ
    let fetched: Option<Subscription> = client
        .get_one(EntityType::Subscriptions, subscription.id.as_str())
        .await
        .expect("Failed to get subscription");

    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.id.as_str(), subscription.id.as_str());
    println!("Read subscription: {}", fetched.id.as_str());

    // UPDATE
    let updated: Subscription = client
        .update(
            EntityType::Subscriptions,
            subscription.id.as_str(),
            &json!({"txnDescription": "Updated subscription description"}),
        )
        .await
        .expect("Failed to update subscription");

    assert_eq!(
        updated.txn_description.as_deref(),
        Some("Updated subscription description")
    );
    println!(
        "Updated subscription description: {:?}",
        updated.txn_description
    );

    // DELETE (deactivate/cancel)
    let cancelled: Subscription = client
        .update(
            EntityType::Subscriptions,
            subscription.id.as_str(),
            &json!({"inactive": 1}),
        )
        .await
        .expect("Failed to cancel subscription");

    assert!(cancelled.inactive);
    println!("Cancelled subscription: {}", cancelled.id.as_str());

    // Cleanup
    let _ = client
        .update::<_, Plan>(EntityType::Plans, plan.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Token>(EntityType::Tokens, token.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Customer>(
            EntityType::Customers,
            customer.id.as_str(),
            &json!({"inactive": 1}),
        )
        .await;

    println!("\n=== SUBSCRIPTION CRUD TEST COMPLETE ===\n");
}

// =============================================================================
// Expanded Type Tests
// =============================================================================

/// Test `get_subscription_expanded()` convenience method.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_subscription_expanded() {
    init_logging();
    let client = create_client();

    // First get subscriptions to find one to test with
    let subs: Vec<Value> = client
        .get_all(EntityType::Subscriptions)
        .await
        .expect("Failed to get subscriptions");

    if subs.is_empty() {
        println!("No subscriptions available for testing");
        return;
    }

    let sub_id = subs[0]["id"].as_str().expect("Subscription should have id");
    println!("Testing get_subscription_expanded with: {}", sub_id);

    // Test the convenience method
    let result = client.get_subscription_expanded(sub_id).await;

    match result {
        Ok(Some(sub)) => {
            println!("Successfully fetched SubscriptionExpanded:");
            println!("  ID: {}", sub.id.as_str());
            println!("  Start: {:?}", sub.start);
            println!("  Finish: {:?}", sub.finish);
            println!("  Failures: {:?}", sub.failures);
            println!("  Max Failures: {:?}", sub.max_failures);
            println!("  Inactive: {}", sub.inactive);

            // Verify expanded plan
            if let Some(ref plan) = sub.plan {
                println!("  Plan EXPANDED:");
                println!("    ID: {}", plan.id.as_str());
                println!("    Name: {:?}", plan.name);
                println!("    Description: {:?}", plan.description);
                println!("    Amount: {:?} (${:.2})", plan.amount, plan.amount.unwrap_or(0) as f64 / 100.0);
                println!("    Schedule: {:?}", plan.schedule);
                println!("    Schedule Factor: {:?}", plan.schedule_factor);
                println!("    Type: {:?}", plan.plan_type);

                // Verify plan ID matches
                assert!(plan.id.as_str().starts_with("t1_pln_"));
            } else {
                println!("  Plan: NOT EXPANDED (may not have a plan)");
            }

            // Test convenience methods
            if let Some(dollars) = sub.plan_amount_dollars() {
                println!("  plan_amount_dollars(): ${:.2}", dollars);
            }
            if let Some(name) = sub.plan_name() {
                println!("  plan_name(): {}", name);
            }
        }
        Ok(None) => println!("Subscription not found"),
        Err(e) => panic!("Failed to fetch subscription: {:?}", e),
    }
}

/// Test SubscriptionExpanded deserialization with raw JSON comparison.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_subscription_expanded_vs_raw_json() {
    init_logging();
    let client = create_client();

    let subs: Vec<Value> = client
        .get_all(EntityType::Subscriptions)
        .await
        .expect("Failed to get subscriptions");

    if subs.is_empty() {
        println!("No subscriptions available");
        return;
    }

    let sub_id = subs[0]["id"].as_str().expect("Subscription should have id");

    // Get raw JSON first
    let raw: Option<Value> = client
        .get_one_expanded(EntityType::Subscriptions, sub_id, &["plan"])
        .await
        .expect("Failed to fetch raw");

    if let Some(ref json) = raw {
        println!("Raw JSON with plan expansion:");
        println!("{}", serde_json::to_string_pretty(json).unwrap());

        // Check what the raw plan field looks like
        if let Some(plan_value) = json.get("plan") {
            println!("\nPlan field in raw JSON:");
            if plan_value.is_object() {
                println!("  -> Is an OBJECT (expansion worked)");
                println!("  -> Has {} fields", plan_value.as_object().unwrap().len());
            } else if plan_value.is_string() {
                println!("  -> Is a STRING: {}", plan_value.as_str().unwrap());
            } else if plan_value.is_null() {
                println!("  -> Is NULL");
            }
        }
    }

    // Now deserialize to SubscriptionExpanded
    let expanded: Option<SubscriptionExpanded> = client
        .get_one_expanded(EntityType::Subscriptions, sub_id, &["plan"])
        .await
        .expect("Failed to fetch expanded");

    if let Some(sub) = expanded {
        println!("\nDeserialized SubscriptionExpanded:");
        println!("  Plan is_some: {}", sub.plan.is_some());
        if let Some(ref plan) = sub.plan {
            println!("  Plan ID: {}", plan.id.as_str());
            println!("  Plan name: {:?}", plan.name);
        }
    }
}

/// Test `get_plan_expanded()` convenience method.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_plan_expanded() {
    init_logging();
    let client = create_client();

    let plans: Vec<Value> = client
        .get_all(EntityType::Plans)
        .await
        .expect("Failed to get plans");

    if plans.is_empty() {
        println!("No plans available for testing");
        return;
    }

    let plan_id = plans[0]["id"].as_str().expect("Plan should have id");
    println!("Testing get_plan_expanded with: {}", plan_id);

    let result = client.get_plan_expanded(plan_id).await;

    match result {
        Ok(Some(plan)) => {
            println!("Successfully fetched PlanExpanded:");
            println!("  ID: {}", plan.id.as_str());
            println!("  Name: {:?}", plan.name);
            println!("  Description: {:?}", plan.description);
            println!("  Amount: ${:.2}", plan.amount_dollars());
            println!("  Schedule: {:?}", plan.schedule);
            println!("  Schedule Factor: {:?}", plan.schedule_factor);
            println!("  Type: {:?}", plan.plan_type);
            println!("  Inactive: {}", plan.inactive);

            // Verify merchant ID (not expanded - just an ID)
            if let Some(ref merchant) = plan.merchant {
                println!("  Merchant ID: {}", merchant.as_str());
                assert!(merchant.as_str().starts_with("t1_mer_"));
            } else {
                println!("  Merchant: NOT PRESENT");
            }

            // Verify expanded subscriptions
            if let Some(ref subs) = plan.subscriptions {
                println!("  Subscriptions EXPANDED: {} total", subs.len());
                for (i, sub) in subs.iter().take(3).enumerate() {
                    println!("    [{}] {} - start: {:?}, inactive: {}",
                        i, sub.id.as_str(), sub.start, sub.inactive);
                }
                if subs.len() > 3 {
                    println!("    ... and {} more", subs.len() - 3);
                }
            } else {
                println!("  Subscriptions: NOT EXPANDED or empty");
            }

            // Test convenience methods
            println!("  amount_dollars(): ${:.2}", plan.amount_dollars());
            println!("  subscription_count(): {}", plan.subscription_count());
        }
        Ok(None) => println!("Plan not found"),
        Err(e) => panic!("Failed to fetch plan: {:?}", e),
    }
}

/// Test PlanExpanded deserialization with raw JSON comparison.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_plan_expanded_vs_raw_json() {
    init_logging();
    let client = create_client();

    let plans: Vec<Value> = client
        .get_all(EntityType::Plans)
        .await
        .expect("Failed to get plans");

    if plans.is_empty() {
        println!("No plans available");
        return;
    }

    let plan_id = plans[0]["id"].as_str().expect("Plan should have id");

    // Get raw JSON first
    let raw: Option<Value> = client
        .get_one_expanded(EntityType::Plans, plan_id, &["merchant", "subscriptions"])
        .await
        .expect("Failed to fetch raw");

    if let Some(ref json) = raw {
        println!("Raw JSON with merchant and subscriptions expansion:");
        println!("{}", serde_json::to_string_pretty(json).unwrap());

        // Check merchant field
        if let Some(merchant_value) = json.get("merchant") {
            println!("\nMerchant field:");
            if merchant_value.is_object() {
                println!("  -> Is an OBJECT (expansion worked)");
            } else if merchant_value.is_string() {
                println!("  -> Is a STRING: {}", merchant_value.as_str().unwrap());
            }
        }

        // Check subscriptions field
        if let Some(subs_value) = json.get("subscriptions") {
            println!("\nSubscriptions field:");
            if let Some(arr) = subs_value.as_array() {
                println!("  -> Is an ARRAY with {} items (expansion worked)", arr.len());
            } else if subs_value.is_null() {
                println!("  -> Is NULL");
            }
        }
    }

    // Deserialize to PlanExpanded
    let expanded: Option<PlanExpanded> = client
        .get_one_expanded(EntityType::Plans, plan_id, &["merchant", "subscriptions"])
        .await
        .expect("Failed to fetch expanded");

    if let Some(plan) = expanded {
        println!("\nDeserialized PlanExpanded:");
        println!("  merchant is_some: {}", plan.merchant.is_some());
        println!("  subscriptions count: {:?}", plan.subscriptions.as_ref().map(|s| s.len()));
    }
}

/// Test fetching multiple subscriptions and their expanded data.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_multiple_subscriptions_expanded() {
    init_logging();
    let client = create_client();

    let subs: Vec<Value> = client
        .get_all(EntityType::Subscriptions)
        .await
        .expect("Failed to get subscriptions");

    println!("Testing expansion on {} subscriptions", subs.len().min(5));

    for sub_json in subs.iter().take(5) {
        let sub_id = sub_json["id"].as_str().expect("Subscription should have id");

        let expanded = client.get_subscription_expanded(sub_id).await;

        match expanded {
            Ok(Some(sub)) => {
                let plan_info = sub.plan.as_ref()
                    .map(|p| format!("{} - ${:.2}", p.name.as_deref().unwrap_or("Unnamed"), p.amount.unwrap_or(0) as f64 / 100.0))
                    .unwrap_or_else(|| "No plan".to_string());

                println!("  {} -> Plan: {}", sub.id.as_str(), plan_info);
            }
            Ok(None) => println!("  {} -> Not found", sub_id),
            Err(e) => println!("  {} -> Error: {:?}", sub_id, e),
        }
    }
}
