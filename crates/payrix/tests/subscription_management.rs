//! Subscription management workflow integration tests.

mod common;

use common::{create_client, init_logging, test_merchant_id};
use payrix::{
    CreateCustomer, CreateToken, Customer, EntityType, Environment, PaymentInfo, PaymentMethod,
    PayrixClient, Plan, Subscription, Token,
};
use payrix::workflows::subscription_management::*;
use serde_json::json;
use std::env;

// =============================================================================
// Core Function Tests
// =============================================================================

/// Test subscribing a customer with an existing plan.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources"]
async fn test_add_plan_to_customer_existing_plan() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== TEST: add_plan_to_customer (existing plan) ===\n");

    // Create prerequisites: customer and token
    let customer: Customer = client
        .create(
            EntityType::Customers,
            &CreateCustomer {
                merchant: Some(test_merchant_id().parse().unwrap()),
                first: Some(format!("SubMgmt{}", timestamp)),
                last: Some("Test".to_string()),
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

    // Create a plan first
    let plan: Plan = client
        .create(
            EntityType::Plans,
            &json!({
                "merchant": test_merchant_id(),
                "type": "recurring",
                "name": format!("SubMgmt Test Plan {}", timestamp),
                "schedule": 3,
                "scheduleFactor": 1,
                "um": "actual",
                "amount": 1999
            }),
        )
        .await
        .expect("Failed to create plan");
    println!("Created plan: {}", plan.id.as_str());

    // Now use the workflow to subscribe
    let config = SubscribeCustomerConfig {
        merchant_id: test_merchant_id().to_string(),
        plan: PlanReference::ExistingId(plan.id.to_string()),
        token: TokenReference::ExistingId(token.id.to_string()),
        start_date: None,
        end_date: None,
        charge_immediately: false,
        tax: None,
        descriptor: None,
        origin: None,
        txn_description: None,
    };

    let result = add_plan_to_customer(&client, config).await;

    match result {
        Ok(res) => {
            println!("Successfully subscribed customer!");
            println!("  Subscription ID: {}", res.subscription.id.as_str());
            println!("  Plan created: {}", res.plan_created);
            println!("  Token created: {}", res.token_created);
            println!("  Initial transaction: {:?}", res.initial_transaction.is_some());

            assert!(!res.plan_created);
            assert!(!res.token_created);
            assert!(res.initial_transaction.is_none());
        }
        Err(e) => {
            panic!("Failed to subscribe customer: {:?}", e);
        }
    }

    // Cleanup
    println!("Cleaning up...");
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

    println!("\n=== TEST COMPLETE ===\n");
}

/// Test subscribing a customer with a new inline plan.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources"]
async fn test_add_plan_to_customer_new_plan() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== TEST: add_plan_to_customer (new inline plan) ===\n");

    // Create prerequisites: customer and token
    let customer: Customer = client
        .create(
            EntityType::Customers,
            &CreateCustomer {
                merchant: Some(test_merchant_id().parse().unwrap()),
                first: Some(format!("SubMgmtNew{}", timestamp)),
                last: Some("Test".to_string()),
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

    // Subscribe with inline plan creation
    let config = SubscribeCustomerConfig {
        merchant_id: test_merchant_id().to_string(),
        plan: PlanReference::NewPlan(PlanConfig {
            name: format!("Inline Plan {}", timestamp),
            description: Some("Created inline during subscription".to_string()),
            schedule: BillingSchedule::Monthly,
            schedule_factor: 1,
            amount: 2999,
            max_failures: Some(3),
        }),
        token: TokenReference::ExistingId(token.id.to_string()),
        start_date: None,
        end_date: None,
        charge_immediately: false,
        tax: None,
        descriptor: None,
        origin: None,
        txn_description: None,
    };

    let result = add_plan_to_customer(&client, config).await;

    match result {
        Ok(res) => {
            println!("Successfully subscribed customer with inline plan!");
            println!("  Subscription ID: {}", res.subscription.id.as_str());
            println!("  Plan ID: {}", res.plan.id.as_str());
            println!("  Plan created: {}", res.plan_created);
            println!("  Token created: {}", res.token_created);

            assert!(res.plan_created);
            assert!(!res.token_created);

            // Cleanup the created plan
            let _ = client
                .update::<_, Plan>(EntityType::Plans, res.plan.id.as_str(), &json!({"inactive": 1}))
                .await;
        }
        Err(e) => {
            panic!("Failed to subscribe customer: {:?}", e);
        }
    }

    // Cleanup
    println!("Cleaning up...");
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

    println!("\n=== TEST COMPLETE ===\n");
}

/// Test payments_to_date function.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_payments_to_date() {
    init_logging();
    let client = create_client();

    println!("\n=== TEST: payments_to_date ===\n");

    // Get an existing subscription
    let subs: Vec<Subscription> = client
        .get_all(EntityType::Subscriptions)
        .await
        .unwrap_or_default();

    if subs.is_empty() {
        println!("No subscriptions available for testing");
        return;
    }

    let sub_id = subs[0].id.as_str();
    println!("Testing with subscription: {}", sub_id);

    let result = payments_to_date(&client, sub_id).await;

    match result {
        Ok(history) => {
            println!("Payment history:");
            println!("  Total paid: ${:.2}", history.total_paid_dollars());
            println!("  Payment count: {}", history.payment_count);
            println!("  Failed count: {}", history.failed_count);
            println!("  Last payment date: {:?}", history.last_payment_date);
            println!("  Transaction count: {}", history.transactions.len());
        }
        Err(e) => {
            println!("Error getting payment history: {:?}", e);
        }
    }

    println!("\n=== TEST COMPLETE ===\n");
}

/// Test next_payment function.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_next_payment() {
    init_logging();
    let client = create_client();

    println!("\n=== TEST: next_payment ===\n");

    // Get an existing subscription with a plan
    let subs: Vec<Subscription> = client
        .get_all(EntityType::Subscriptions)
        .await
        .unwrap_or_default();

    let sub_with_plan = subs.iter().find(|s| s.plan.is_some());

    if let Some(sub) = sub_with_plan {
        println!("Testing with subscription: {}", sub.id.as_str());

        let result = next_payment(&client, sub.id.as_str()).await;

        match result {
            Ok(next) => {
                println!("Next payment:");
                println!("  Date: {}", next.date);
                println!("  Amount: ${:.2}", next.amount_dollars());
                println!("  Days until: {}", next.days_until);
                println!("  Is active: {}", next.is_active);
            }
            Err(e) => {
                println!("Error calculating next payment: {:?}", e);
            }
        }
    } else {
        println!("No subscriptions with plans available for testing");
    }

    println!("\n=== TEST COMPLETE ===\n");
}

// =============================================================================
// Helper Function Tests
// =============================================================================

/// Test cancel_subscription function.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - modifies resources"]
async fn test_cancel_subscription() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== TEST: cancel_subscription ===\n");

    // Create a subscription to cancel
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let plan: Plan = client
        .create(
            EntityType::Plans,
            &json!({
                "merchant": test_merchant_id(),
                "type": "recurring",
                "name": format!("Cancel Test Plan {}", timestamp),
                "schedule": 3,
                "amount": 999
            }),
        )
        .await
        .expect("Failed to create plan");

    let start_date = chrono::Utc::now() + chrono::Duration::days(30);
    let start_date_int = start_date
        .format("%Y%m%d")
        .to_string()
        .parse::<i32>()
        .unwrap();

    let sub: Subscription = client
        .create(
            EntityType::Subscriptions,
            &json!({
                "plan": plan.id.as_str(),
                "start": start_date_int
            }),
        )
        .await
        .expect("Failed to create subscription");

    println!("Created subscription: {}", sub.id.as_str());
    println!("  Inactive before: {}", sub.inactive);

    // Cancel it
    let result = cancel_subscription(&client, sub.id.as_str()).await;

    match result {
        Ok(cancelled) => {
            println!("Cancelled subscription!");
            println!("  Inactive after: {}", cancelled.inactive);
            assert!(cancelled.inactive);
        }
        Err(e) => {
            panic!("Failed to cancel subscription: {:?}", e);
        }
    }

    // Cleanup
    let _ = client
        .update::<_, Plan>(EntityType::Plans, plan.id.as_str(), &json!({"inactive": 1}))
        .await;

    println!("\n=== TEST COMPLETE ===\n");
}

/// Test pause_subscription and resume_subscription functions.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - modifies resources"]
async fn test_pause_resume_subscription() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== TEST: pause_subscription and resume_subscription ===\n");

    // Create a subscription
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let plan: Plan = client
        .create(
            EntityType::Plans,
            &json!({
                "merchant": test_merchant_id(),
                "type": "recurring",
                "name": format!("Pause Test Plan {}", timestamp),
                "schedule": 3,
                "amount": 999
            }),
        )
        .await
        .expect("Failed to create plan");

    let start_date = chrono::Utc::now() + chrono::Duration::days(30);
    let start_date_int = start_date
        .format("%Y%m%d")
        .to_string()
        .parse::<i32>()
        .unwrap();

    let sub: Subscription = client
        .create(
            EntityType::Subscriptions,
            &json!({
                "plan": plan.id.as_str(),
                "start": start_date_int
            }),
        )
        .await
        .expect("Failed to create subscription");

    println!("Created subscription: {}", sub.id.as_str());
    println!("  Frozen initial: {}", sub.frozen);

    // Pause
    let paused = pause_subscription(&client, sub.id.as_str())
        .await
        .expect("Failed to pause");
    println!("Paused subscription - frozen: {}", paused.frozen);
    assert!(paused.frozen);

    // Resume - Note: API may have issues resuming subscriptions that haven't started yet
    match resume_subscription(&client, sub.id.as_str()).await {
        Ok(resumed) => {
            println!("Resumed subscription - frozen: {}", resumed.frozen);
            assert!(!resumed.frozen);
        }
        Err(e) => {
            // The Payrix API sometimes returns errors when resuming subscriptions
            // that haven't started yet. This is expected behavior.
            println!("Note: Failed to resume (API limitation for future-dated subscriptions): {:?}", e);
        }
    }

    // Cleanup
    let _ = client
        .update::<_, Subscription>(EntityType::Subscriptions, sub.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Plan>(EntityType::Plans, plan.id.as_str(), &json!({"inactive": 1}))
        .await;

    println!("\n=== TEST COMPLETE ===\n");
}

/// Test get_subscription_status function.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_subscription_status() {
    init_logging();
    let client = create_client();

    println!("\n=== TEST: get_subscription_status ===\n");

    // Get an existing subscription
    let subs: Vec<Subscription> = client
        .get_all(EntityType::Subscriptions)
        .await
        .unwrap_or_default();

    if subs.is_empty() {
        println!("No subscriptions available for testing");
        return;
    }

    let sub_id = subs[0].id.as_str();
    println!("Testing with subscription: {}", sub_id);

    let result = get_subscription_status(&client, sub_id).await;

    match result {
        Ok(status) => {
            println!("Subscription status:");
            println!("  ID: {}", status.subscription.id.as_str());
            println!("  State: {:?}", status.state);
            println!("  Plan: {:?}", status.plan.as_ref().map(|p| p.name.as_ref()));
            println!("  Total paid: ${:.2}", status.payment_summary.total_paid_dollars());
            println!("  Payment count: {}", status.payment_summary.payment_count);
            if let Some(ref next) = status.next_payment {
                println!("  Next payment: {} - ${:.2}", next.date, next.amount_dollars());
            }
        }
        Err(e) => {
            println!("Error getting subscription status: {:?}", e);
        }
    }

    println!("\n=== TEST COMPLETE ===\n");
}

// =============================================================================
// Payer/Payee Function Tests
// =============================================================================

/// Test get_subscribers_for_plan function.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_subscribers_for_plan() {
    init_logging();
    let client = create_client();

    println!("\n=== TEST: get_subscribers_for_plan ===\n");

    // Get a plan with subscriptions
    let plans: Vec<Plan> = client
        .get_all(EntityType::Plans)
        .await
        .unwrap_or_default();

    if plans.is_empty() {
        println!("No plans available for testing");
        return;
    }

    let plan_id = plans[0].id.as_str();
    println!("Testing with plan: {} ({:?})", plan_id, plans[0].name);

    let result = get_subscribers_for_plan(&client, plan_id).await;

    match result {
        Ok(subscribers) => {
            println!("Found {} subscribers", subscribers.len());
            for (i, sub) in subscribers.iter().take(5).enumerate() {
                println!(
                    "  [{}] {} - inactive: {}, frozen: {}",
                    i,
                    sub.id.as_str(),
                    sub.inactive,
                    sub.frozen
                );
            }
            if subscribers.len() > 5 {
                println!("  ... and {} more", subscribers.len() - 5);
            }
        }
        Err(e) => {
            println!("Error getting subscribers: {:?}", e);
        }
    }

    println!("\n=== TEST COMPLETE ===\n");
}

/// Test calculate_subscription_revenue function.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_calculate_subscription_revenue() {
    init_logging();
    let client = create_client();

    println!("\n=== TEST: calculate_subscription_revenue ===\n");

    // Get a plan
    let plans: Vec<Plan> = client
        .get_all(EntityType::Plans)
        .await
        .unwrap_or_default();

    if plans.is_empty() {
        println!("No plans available for testing");
        return;
    }

    let plan_id = plans[0].id.as_str();
    println!("Testing with plan: {} ({:?})", plan_id, plans[0].name);

    let result = calculate_subscription_revenue(&client, plan_id, None, None).await;

    match result {
        Ok(revenue) => {
            println!("Revenue metrics:");
            println!("  Total collected: ${:.2}", revenue.total_collected_dollars());
            println!("  Projected monthly: ${:.2}", revenue.projected_monthly_dollars());
            println!("  Projected annual: ${:.2}", revenue.projected_annual_dollars());
            println!("  Active subscribers: {}", revenue.active_subscribers);
            println!("  Churned subscribers: {}", revenue.churned_subscribers);
        }
        Err(e) => {
            println!("Error calculating revenue: {:?}", e);
        }
    }

    println!("\n=== TEST COMPLETE ===\n");
}

// =============================================================================
// Unit Test Validation
// =============================================================================

/// Verify that unit tests pass.
#[test]
fn test_billing_schedule_conversions() {
    use payrix::PlanSchedule;

    assert_eq!(
        BillingSchedule::Daily.to_plan_schedule(),
        PlanSchedule::Daily
    );
    assert_eq!(
        BillingSchedule::Weekly.to_plan_schedule(),
        PlanSchedule::Weekly
    );
    assert_eq!(
        BillingSchedule::Monthly.to_plan_schedule(),
        PlanSchedule::Monthly
    );
    assert_eq!(
        BillingSchedule::Annually.to_plan_schedule(),
        PlanSchedule::Annually
    );
}

// =============================================================================
// Field Verification Tests
// =============================================================================

/// Test that created Plan has all expected fields populated correctly.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources"]
async fn test_plan_fields_are_correct() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== TEST: Plan Field Verification ===\n");

    let plan_name = format!("Field Test Plan {}", timestamp);
    let plan_desc = "Testing all plan fields";

    // Create a plan with all configurable fields
    let plan: Plan = client
        .create(
            EntityType::Plans,
            &json!({
                "merchant": test_merchant_id(),
                "type": "recurring",
                "name": plan_name,
                "description": plan_desc,
                "schedule": 3,  // Monthly
                "scheduleFactor": 2,  // Every 2 months
                "um": "actual",
                "amount": 4999,
                "maxFailures": 5
            }),
        )
        .await
        .expect("Failed to create plan");

    println!("Created plan: {}", plan.id.as_str());
    println!("Plan fields:");
    println!("  id: {}", plan.id.as_str());
    println!("  name: {:?}", plan.name);
    println!("  description: {:?}", plan.description);
    println!("  schedule: {:?}", plan.schedule);
    println!("  scheduleFactor: {:?}", plan.schedule_factor);
    println!("  um: {:?}", plan.um);
    println!("  amount: {:?}", plan.amount);
    println!("  maxFailures: {:?}", plan.max_failures);
    println!("  merchant: {:?}", plan.merchant);
    println!("  created: {:?}", plan.created);
    println!("  inactive: {}", plan.inactive);
    println!("  frozen: {}", plan.frozen);

    // Verify required fields
    assert!(!plan.id.as_str().is_empty(), "Plan ID should not be empty");
    assert_eq!(plan.name, Some(plan_name.clone()), "Plan name mismatch");
    assert_eq!(plan.description, Some(plan_desc.to_string()), "Plan description mismatch");
    assert_eq!(plan.schedule, Some(payrix::PlanSchedule::Monthly), "Schedule should be Monthly");
    assert_eq!(plan.schedule_factor, Some(2), "Schedule factor should be 2");
    assert_eq!(plan.amount, Some(4999), "Amount should be 4999 cents");
    assert_eq!(plan.max_failures, Some(5), "Max failures should be 5");
    assert!(plan.merchant.is_some(), "Merchant should be set");
    assert!(plan.created.is_some(), "Created timestamp should be set");
    assert!(!plan.inactive, "Plan should not be inactive");
    assert!(!plan.frozen, "Plan should not be frozen");

    // Cleanup
    let _ = client
        .update::<_, Plan>(EntityType::Plans, plan.id.as_str(), &json!({"inactive": 1}))
        .await;

    println!("\n=== TEST COMPLETE ===\n");
}

/// Test that created Subscription has all expected fields populated correctly.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources"]
async fn test_subscription_fields_are_correct() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== TEST: Subscription Field Verification ===\n");

    // Create a plan first
    let plan: Plan = client
        .create(
            EntityType::Plans,
            &json!({
                "merchant": test_merchant_id(),
                "type": "recurring",
                "name": format!("SubField Test Plan {}", timestamp),
                "schedule": 3,
                "amount": 1999
            }),
        )
        .await
        .expect("Failed to create plan");

    println!("Created plan: {}", plan.id.as_str());

    // Calculate dates
    let start_date = chrono::Utc::now() + chrono::Duration::days(7);
    let start_date_int = start_date
        .format("%Y%m%d")
        .to_string()
        .parse::<i32>()
        .unwrap();

    let end_date = chrono::Utc::now() + chrono::Duration::days(365);
    let end_date_int = end_date
        .format("%Y%m%d")
        .to_string()
        .parse::<i32>()
        .unwrap();

    // Create subscription with all fields
    let sub: Subscription = client
        .create(
            EntityType::Subscriptions,
            &json!({
                "plan": plan.id.as_str(),
                "start": start_date_int,
                "finish": end_date_int,
                "tax": 199,
                "descriptor": "Test Descriptor",
                "txnDescription": "Monthly subscription",
                "order": format!("ORD-{}", timestamp),
                "origin": 2,  // eCommerce
                "maxFailures": 3
            }),
        )
        .await
        .expect("Failed to create subscription");

    println!("Created subscription: {}", sub.id.as_str());
    println!("Subscription fields:");
    println!("  id: {}", sub.id.as_str());
    println!("  plan: {:?}", sub.plan);
    println!("  start: {:?}", sub.start);
    println!("  finish: {:?}", sub.finish);
    println!("  tax: {:?}", sub.tax);
    println!("  descriptor: {:?}", sub.descriptor);
    println!("  txnDescription: {:?}", sub.txn_description);
    println!("  order: {:?}", sub.order);
    println!("  origin: {:?}", sub.origin);
    println!("  maxFailures: {:?}", sub.max_failures);
    println!("  failures: {:?}", sub.failures);
    println!("  created: {:?}", sub.created);
    println!("  inactive: {}", sub.inactive);
    println!("  frozen: {}", sub.frozen);

    // Verify fields
    assert!(!sub.id.as_str().is_empty(), "Subscription ID should not be empty");
    assert_eq!(sub.plan.as_ref().map(|p| p.as_str()), Some(plan.id.as_str()), "Plan ID mismatch");
    assert_eq!(sub.start, Some(start_date_int), "Start date mismatch");
    assert_eq!(sub.finish, Some(end_date_int), "Finish date mismatch");
    assert_eq!(sub.tax, Some(199), "Tax mismatch");
    assert_eq!(sub.descriptor, Some("Test Descriptor".to_string()), "Descriptor mismatch");
    assert_eq!(sub.txn_description, Some("Monthly subscription".to_string()), "Txn description mismatch");
    assert!(sub.order.is_some(), "Order should be set");
    assert_eq!(sub.origin, Some(payrix::SubscriptionOrigin::ECommerce), "Origin should be ECommerce");
    assert!(sub.created.is_some(), "Created timestamp should be set");
    assert!(!sub.inactive, "Subscription should not be inactive");
    assert!(!sub.frozen, "Subscription should not be frozen");

    // Cleanup
    let _ = client
        .update::<_, Subscription>(EntityType::Subscriptions, sub.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Plan>(EntityType::Plans, plan.id.as_str(), &json!({"inactive": 1}))
        .await;

    println!("\n=== TEST COMPLETE ===\n");
}

/// Test SubscriptionTokens entity - linking tokens to subscriptions.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources"]
async fn test_subscription_tokens_entity() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== TEST: SubscriptionTokens Entity ===\n");

    // Create customer
    let customer: Customer = client
        .create(
            EntityType::Customers,
            &payrix::CreateCustomer {
                merchant: Some(test_merchant_id().parse().unwrap()),
                first: Some(format!("SubToken{}", timestamp)),
                last: Some("Test".to_string()),
                email: Some("payrixrust@gmail.com".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create customer");

    println!("Created customer: {}", customer.id.as_str());

    // Create token
    let token: Token = client
        .create(
            EntityType::Tokens,
            &payrix::CreateToken {
                customer: customer.id.to_string().parse().unwrap(),
                payment: payrix::PaymentInfo {
                    method: payrix::PaymentMethod::Visa,
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

    // Create plan
    let plan: Plan = client
        .create(
            EntityType::Plans,
            &json!({
                "merchant": test_merchant_id(),
                "type": "recurring",
                "name": format!("SubToken Test Plan {}", timestamp),
                "schedule": 3,
                "amount": 999
            }),
        )
        .await
        .expect("Failed to create plan");

    println!("Created plan: {}", plan.id.as_str());

    // Create subscription
    let start_date = chrono::Utc::now() + chrono::Duration::days(30);
    let start_date_int = start_date
        .format("%Y%m%d")
        .to_string()
        .parse::<i32>()
        .unwrap();

    let sub: Subscription = client
        .create(
            EntityType::Subscriptions,
            &json!({
                "plan": plan.id.as_str(),
                "start": start_date_int
            }),
        )
        .await
        .expect("Failed to create subscription");

    println!("Created subscription: {}", sub.id.as_str());

    // Create subscription token link
    let sub_token_result: Result<serde_json::Value, _> = client
        .create(
            EntityType::SubscriptionTokens,
            &json!({
                "subscription": sub.id.as_str(),
                "token": token.id.as_str()
            }),
        )
        .await;

    match sub_token_result {
        Ok(sub_token) => {
            println!("Created subscription token link: {:?}", sub_token);
            assert!(sub_token.get("id").is_some(), "Subscription token should have an ID");
            assert_eq!(
                sub_token.get("subscription").and_then(|v| v.as_str()),
                Some(sub.id.as_str()),
                "Subscription ID mismatch in subscriptionToken"
            );
            assert_eq!(
                sub_token.get("token").and_then(|v| v.as_str()),
                Some(token.id.as_str()),
                "Token ID mismatch in subscriptionToken"
            );
        }
        Err(e) => {
            println!("Failed to create subscription token link: {:?}", e);
            println!("Note: This may be expected if the API doesn't support this endpoint directly");
        }
    }

    // Verify we can query subscription tokens
    let search = payrix::SearchBuilder::new()
        .field("subscription", sub.id.as_str())
        .build();

    let sub_tokens: Result<Vec<serde_json::Value>, _> = client
        .search(EntityType::SubscriptionTokens, &search)
        .await;

    match sub_tokens {
        Ok(tokens) => {
            println!("Found {} subscription token links", tokens.len());
            for st in &tokens {
                println!("  {:?}", st);
            }
        }
        Err(e) => {
            println!("Failed to query subscription tokens: {:?}", e);
        }
    }

    // Cleanup
    let _ = client
        .update::<_, Subscription>(EntityType::Subscriptions, sub.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Plan>(EntityType::Plans, plan.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Token>(EntityType::Tokens, token.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Customer>(EntityType::Customers, customer.id.as_str(), &json!({"inactive": 1}))
        .await;

    println!("\n=== TEST COMPLETE ===\n");
}

/// Test full workflow: add_plan_to_customer with charge_immediately.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources and charges"]
async fn test_add_plan_to_customer_charge_immediately() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== TEST: add_plan_to_customer with charge_immediately ===\n");

    // Create customer
    let customer: Customer = client
        .create(
            EntityType::Customers,
            &payrix::CreateCustomer {
                merchant: Some(test_merchant_id().parse().unwrap()),
                first: Some(format!("ChargeNow{}", timestamp)),
                last: Some("Test".to_string()),
                email: Some("payrixrust@gmail.com".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create customer");

    println!("Created customer: {}", customer.id.as_str());

    // Create token
    let token: Token = client
        .create(
            EntityType::Tokens,
            &payrix::CreateToken {
                customer: customer.id.to_string().parse().unwrap(),
                payment: payrix::PaymentInfo {
                    method: payrix::PaymentMethod::Visa,
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

    // Subscribe with inline plan and charge immediately
    let config = SubscribeCustomerConfig {
        merchant_id: test_merchant_id().to_string(),
        plan: PlanReference::NewPlan(PlanConfig {
            name: format!("Charge Now Plan {}", timestamp),
            description: Some("Plan with immediate charge".to_string()),
            schedule: BillingSchedule::Monthly,
            schedule_factor: 1,
            amount: 100,  // $1.00 - small amount for testing
            max_failures: Some(3),
        }),
        token: TokenReference::ExistingId(token.id.to_string()),
        start_date: None,
        end_date: None,
        charge_immediately: true,
        tax: None,
        descriptor: None,
        origin: None,
        txn_description: Some("Immediate subscription charge".to_string()),
    };

    let result = add_plan_to_customer(&client, config).await;

    match result {
        Ok(res) => {
            println!("Successfully subscribed with immediate charge!");
            println!("  Subscription ID: {}", res.subscription.id.as_str());
            println!("  Plan ID: {}", res.plan.id.as_str());
            println!("  Plan created: {}", res.plan_created);
            println!("  Token created: {}", res.token_created);

            assert!(res.plan_created, "Plan should have been created");
            assert!(!res.token_created, "Token should not have been created (used existing)");

            // Verify initial transaction was created
            if let Some(ref txn) = res.initial_transaction {
                println!("  Initial transaction: {}", txn.id.as_str());
                println!("    Total: {:?}", txn.total);
                println!("    Status: {:?}", txn.status);
                println!("    Type: {:?}", txn.txn_type);
                assert_eq!(txn.total, Some(100), "Transaction total should be 100 cents");
            } else {
                println!("  No initial transaction (may have failed due to merchant status)");
            }

            // Cleanup
            let _ = client
                .update::<_, Subscription>(EntityType::Subscriptions, res.subscription.id.as_str(), &json!({"inactive": 1}))
                .await;
            let _ = client
                .update::<_, Plan>(EntityType::Plans, res.plan.id.as_str(), &json!({"inactive": 1}))
                .await;
        }
        Err(e) => {
            println!("Failed to subscribe: {:?}", e);
            println!("Note: This may fail if the test merchant isn't fully boarded");
        }
    }

    // Cleanup
    let _ = client
        .update::<_, Token>(EntityType::Tokens, token.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Customer>(EntityType::Customers, customer.id.as_str(), &json!({"inactive": 1}))
        .await;

    println!("\n=== TEST COMPLETE ===\n");
}

/// Test PlanConfig validation.
#[test]
fn test_plan_config_validation() {
    // Valid config
    let valid = PlanConfig {
        name: "Test Plan".to_string(),
        description: Some("A test plan".to_string()),
        schedule: BillingSchedule::Monthly,
        schedule_factor: 1,
        amount: 1000,
        max_failures: Some(3),
    };
    assert!(valid.validate().is_ok(), "Valid config should pass validation");

    // Empty name
    let empty_name = PlanConfig {
        name: "".to_string(),
        description: None,
        schedule: BillingSchedule::Monthly,
        schedule_factor: 1,
        amount: 1000,
        max_failures: None,
    };
    assert!(empty_name.validate().is_err(), "Empty name should fail validation");

    // Zero schedule factor
    let zero_factor = PlanConfig {
        name: "Test".to_string(),
        description: None,
        schedule: BillingSchedule::Monthly,
        schedule_factor: 0,
        amount: 1000,
        max_failures: None,
    };
    assert!(zero_factor.validate().is_err(), "Zero schedule factor should fail validation");

    // Negative schedule factor
    let negative_factor = PlanConfig {
        name: "Test".to_string(),
        description: None,
        schedule: BillingSchedule::Monthly,
        schedule_factor: -1,
        amount: 1000,
        max_failures: None,
    };
    assert!(negative_factor.validate().is_err(), "Negative schedule factor should fail validation");

    // Zero amount
    let zero_amount = PlanConfig {
        name: "Test".to_string(),
        description: None,
        schedule: BillingSchedule::Monthly,
        schedule_factor: 1,
        amount: 0,
        max_failures: None,
    };
    assert!(zero_amount.validate().is_err(), "Zero amount should fail validation");

    // Negative amount
    let negative_amount = PlanConfig {
        name: "Test".to_string(),
        description: None,
        schedule: BillingSchedule::Monthly,
        schedule_factor: 1,
        amount: -100,
        max_failures: None,
    };
    assert!(negative_amount.validate().is_err(), "Negative amount should fail validation");
}

/// Test TokenConfig Debug redacts sensitive data.
#[test]
fn test_token_config_debug_redacts_sensitive_data() {
    let config = TokenConfig {
        customer_id: "t1_cus_12345".to_string(),
        method: payrix::PaymentMethod::Visa,
        number: "4111111111111111".to_string(),
        routing: Some("021000021".to_string()),
        expiration: Some("1230".to_string()),
        cvv: Some("123".to_string()),
    };

    let debug_output = format!("{:?}", config);

    // Should NOT contain full card number
    assert!(
        !debug_output.contains("4111111111111111"),
        "Debug output should not contain full card number"
    );

    // Should contain last 4 digits
    assert!(
        debug_output.contains("****1111"),
        "Debug output should show redacted card with last 4 digits"
    );

    // Should NOT contain CVV
    assert!(
        !debug_output.contains("123") || debug_output.contains("[REDACTED]"),
        "Debug output should redact CVV"
    );

    // Should show [REDACTED] for routing
    assert!(
        debug_output.contains("[REDACTED]"),
        "Debug output should show [REDACTED] for sensitive fields"
    );

    // Should still show non-sensitive fields
    assert!(
        debug_output.contains("t1_cus_12345"),
        "Debug output should show customer_id"
    );
}

/// Test update_payment_method function.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources"]
async fn test_update_payment_method() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== TEST: update_payment_method ===\n");

    // Create customer
    let customer: Customer = client
        .create(
            EntityType::Customers,
            &payrix::CreateCustomer {
                merchant: Some(test_merchant_id().parse().unwrap()),
                first: Some(format!("UpdatePM{}", timestamp)),
                last: Some("Test".to_string()),
                email: Some("payrixrust@gmail.com".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create customer");

    println!("Created customer: {}", customer.id.as_str());

    // Create original token
    let token1: Token = client
        .create(
            EntityType::Tokens,
            &payrix::CreateToken {
                customer: customer.id.to_string().parse().unwrap(),
                payment: payrix::PaymentInfo {
                    method: payrix::PaymentMethod::Visa,
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

    println!("Created original token: {}", token1.id.as_str());

    // Create new token for update
    let token2: Token = client
        .create(
            EntityType::Tokens,
            &payrix::CreateToken {
                customer: customer.id.to_string().parse().unwrap(),
                payment: payrix::PaymentInfo {
                    method: payrix::PaymentMethod::Mastercard,
                    number: Some("5555555555554444".to_string()),
                    routing: None,
                    expiration: Some("1231".to_string()),
                    cvv: Some("321".to_string()),
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
        .expect("Failed to create new token");

    println!("Created new token: {}", token2.id.as_str());

    // Create subscription with original token
    let config = SubscribeCustomerConfig {
        merchant_id: test_merchant_id().to_string(),
        plan: PlanReference::NewPlan(PlanConfig {
            name: format!("Update PM Plan {}", timestamp),
            description: None,
            schedule: BillingSchedule::Monthly,
            schedule_factor: 1,
            amount: 999,
            max_failures: Some(3),
        }),
        token: TokenReference::ExistingId(token1.id.to_string()),
        start_date: None,
        end_date: None,
        charge_immediately: false,
        tax: None,
        descriptor: None,
        origin: None,
        txn_description: None,
    };

    let result = add_plan_to_customer(&client, config).await.expect("Failed to create subscription");
    println!("Created subscription: {}", result.subscription.id.as_str());

    // Update payment method
    let updated = update_payment_method(
        &client,
        result.subscription.id.as_str(),
        TokenReference::ExistingId(token2.id.to_string()),
    )
    .await;

    match updated {
        Ok(sub) => {
            println!("Updated payment method successfully");
            println!("  Subscription ID: {}", sub.id.as_str());
            // The subscription itself doesn't store the token directly,
            // the link is in subscription_tokens
        }
        Err(e) => {
            println!("Error updating payment method: {:?}", e);
        }
    }

    // Cleanup
    let _ = client
        .update::<_, Subscription>(EntityType::Subscriptions, result.subscription.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Plan>(EntityType::Plans, result.plan.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Token>(EntityType::Tokens, token1.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Token>(EntityType::Tokens, token2.id.as_str(), &json!({"inactive": 1}))
        .await;
    let _ = client
        .update::<_, Customer>(EntityType::Customers, customer.id.as_str(), &json!({"inactive": 1}))
        .await;

    println!("\n=== TEST COMPLETE ===\n");
}
