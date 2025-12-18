//! Search and pagination integration tests.

mod common;

use common::{create_client, init_logging};
use payrix::{
    Alert, AlertAction, Customer, EntityType, Environment, Merchant, Note, PayrixClient, Plan,
    SearchBuilder, Subscription,
};
use std::env;

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_search_with_pagination() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    // Get first page
    let (merchants, page_info) = client
        .get_page::<Merchant>(
            EntityType::Merchants,
            1,
            10,
            &std::collections::HashMap::new(),
            None,
        )
        .await
        .expect("Failed to get page");

    println!(
        "Page 1: {} merchants, has_more: {}",
        merchants.len(),
        page_info.has_more
    );

    if page_info.has_more {
        let (page2, _) = client
            .get_page::<Merchant>(
                EntityType::Merchants,
                2,
                10,
                &std::collections::HashMap::new(),
                None,
            )
            .await
            .expect("Failed to get page 2");

        println!("Page 2: {} merchants", page2.len());
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_error_handling() {
    init_logging();
    let client = create_client();

    // Try to get a non-existent resource
    let result: Result<Option<Customer>, _> = client
        .get_one(EntityType::Customers, "t1_cus_nonexistent0000000000000")
        .await;

    // Should return Ok(None) or an error, not panic
    match result {
        Ok(None) => println!("Correctly returned None for non-existent customer"),
        Ok(Some(_)) => panic!("Should not find a non-existent customer"),
        Err(e) => println!("Got expected error: {}", e),
    }
}

/// Test bulk search and pagination across multiple entity types.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_bulk_search_pagination() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== BULK SEARCH PAGINATION TEST ===\n");

    // Test pagination with Plans
    let (plans_page1, page_info) = client
        .get_page::<Plan>(
            EntityType::Plans,
            1,
            5,
            &std::collections::HashMap::new(),
            None,
        )
        .await
        .expect("Failed to get plans page 1");

    println!(
        "Plans page 1: {} items, has_more: {}",
        plans_page1.len(),
        page_info.has_more
    );

    if page_info.has_more {
        let (plans_page2, _) = client
            .get_page::<Plan>(
                EntityType::Plans,
                2,
                5,
                &std::collections::HashMap::new(),
                None,
            )
            .await
            .expect("Failed to get plans page 2");
        println!("Plans page 2: {} items", plans_page2.len());
    }

    // Test pagination with Subscriptions
    let (subs_page1, sub_info) = client
        .get_page::<Subscription>(
            EntityType::Subscriptions,
            1,
            5,
            &std::collections::HashMap::new(),
            None,
        )
        .await
        .expect("Failed to get subscriptions page 1");

    println!(
        "Subscriptions page 1: {} items, has_more: {}",
        subs_page1.len(),
        sub_info.has_more
    );

    // Test pagination with Notes
    let (notes_page1, notes_info) = client
        .get_page::<Note>(
            EntityType::Notes,
            1,
            5,
            &std::collections::HashMap::new(),
            None,
        )
        .await
        .expect("Failed to get notes page 1");

    println!(
        "Notes page 1: {} items, has_more: {}",
        notes_page1.len(),
        notes_info.has_more
    );

    // Test pagination with Alerts
    let (alerts_page1, alerts_info) = client
        .get_page::<Alert>(
            EntityType::Alerts,
            1,
            5,
            &std::collections::HashMap::new(),
            None,
        )
        .await
        .expect("Failed to get alerts page 1");

    println!(
        "Alerts page 1: {} items, has_more: {}",
        alerts_page1.len(),
        alerts_info.has_more
    );

    println!("\n=== BULK SEARCH PAGINATION TEST COMPLETE ===\n");
}

/// Test searching entities with complex filters.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_complex_search_filters() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== COMPLEX SEARCH FILTERS TEST ===\n");

    // Search for active plans with amount > 0
    let search = SearchBuilder::new()
        .field("inactive", "0")
        .field("amount[greater]", "0")
        .build();

    let plans: Vec<Plan> = client
        .search(EntityType::Plans, &search)
        .await
        .expect("Failed to search plans");

    println!("Found {} active plans with amount > 0", plans.len());
    for plan in plans.iter().take(3) {
        println!(
            "  Plan: {} - {:?}, amount: {:?}",
            plan.id.as_str(),
            plan.name,
            plan.amount
        );
    }

    // Search for notes created today
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let search = SearchBuilder::new()
        .field("created[greater]", &format!("{} 00:00:00.0000", today))
        .build();

    let notes: Vec<Note> = client
        .search(EntityType::Notes, &search)
        .await
        .expect("Failed to search notes");

    println!("Found {} notes created today ({})", notes.len(), today);

    // Search for web-type alert actions
    let search = SearchBuilder::new().field("type", "web").build();

    let actions: Vec<AlertAction> = client
        .search(EntityType::AlertActions, &search)
        .await
        .expect("Failed to search alert actions");

    println!("Found {} web-type alert actions", actions.len());
    for action in actions.iter().take(3) {
        println!("  Action: {} - {:?}", action.id.as_str(), action.value);
    }

    println!("\n=== COMPLEX SEARCH FILTERS TEST COMPLETE ===\n");
}
