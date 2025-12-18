//! Plan and Subscription integration tests.

mod common;

use common::{create_client, init_logging, TEST_MERCHANT_ID};
use payrix::{
    CreateCustomer, CreateToken, Customer, EntityType, Environment, PaymentInfo, PaymentMethod,
    PayrixClient, Plan, Subscription, Token,
};
use serde_json::json;
use std::env;

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
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
#[ignore = "requires PAYRIX_API_KEY environment variable"]
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
#[ignore = "requires PAYRIX_API_KEY - creates real resources"]
async fn test_plan_crud() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== PLAN CRUD TEST ===\n");

    // CREATE
    let new_plan = json!({
        "merchant": TEST_MERCHANT_ID,
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
#[ignore = "requires PAYRIX_API_KEY - creates real resources"]
async fn test_subscription_crud() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
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
                merchant: Some(TEST_MERCHANT_ID.parse().unwrap()),
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
                "merchant": TEST_MERCHANT_ID,
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
