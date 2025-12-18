//! Alert and Note integration tests.

mod common;

use common::{create_client, init_logging, test_entity_id};
use payrix::{
    Alert, AlertAction, AlertTrigger, EntityType, Environment, Note, NoteDocument, PayrixClient,
    SearchBuilder,
};
use serde_json::json;
use std::env;

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_alerts() {
    init_logging();
    let client = create_client();

    let alerts: Vec<Alert> = client.get_all(EntityType::Alerts).await.unwrap();

    println!("Found {} alerts", alerts.len());
    for a in alerts.iter().take(5) {
        println!(
            "  Alert: {} - name: {:?}, login: {:?}",
            a.id.as_str(),
            a.name,
            a.login
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_alert_actions() {
    init_logging();
    let client = create_client();

    let actions: Vec<AlertAction> = client.get_all(EntityType::AlertActions).await.unwrap();

    println!("Found {} alert actions", actions.len());
    for a in actions.iter().take(5) {
        println!(
            "  AlertAction: {} - alert: {:?}, action_type: {:?}",
            a.id.as_str(),
            a.alert,
            a.action_type
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_alert_triggers() {
    init_logging();
    let client = create_client();

    let triggers: Vec<AlertTrigger> = client.get_all(EntityType::AlertTriggers).await.unwrap();

    println!("Found {} alert triggers", triggers.len());
    for t in triggers.iter().take(5) {
        println!(
            "  AlertTrigger: {} - alert: {:?}, event: {:?}",
            t.id.as_str(),
            t.alert,
            t.event
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_notes() {
    init_logging();
    let client = create_client();

    let notes: Vec<Note> = client.get_all(EntityType::Notes).await.unwrap();

    println!("Found {} notes", notes.len());
    for n in notes.iter().take(5) {
        println!(
            "  Note: {} - entity: {:?}, note: {:?}, note_type: {:?}",
            n.id.as_str(),
            n.entity,
            n.note,
            n.note_type
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_note_documents() {
    init_logging();
    let client = create_client();

    let docs: Vec<NoteDocument> = client.get_all(EntityType::NoteDocuments).await.unwrap();

    println!("Found {} note documents", docs.len());
    for d in docs.iter().take(5) {
        println!(
            "  NoteDocument: {} - note: {:?}, custom: {:?}, document_type: {:?}",
            d.id.as_str(),
            d.note,
            d.custom,
            d.document_type
        );
    }
}

/// Test Note CRUD operations.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources"]
async fn test_note_crud() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== NOTE CRUD TEST ===\n");

    let entity_id = test_entity_id();
    // CREATE note on our test entity
    let new_note = json!({
        "entity": entity_id,
        "type": "note",
        "note": format!("Integration test note created at {}", timestamp)
    });

    let note: Note = client
        .create(EntityType::Notes, &new_note)
        .await
        .expect("Failed to create note");

    println!("Created note: {}", note.id.as_str());
    println!("  Entity: {:?}", note.entity);
    println!("  Type: {:?}", note.note_type);
    println!("  Note: {:?}", note.note);

    assert!(note.id.as_str().starts_with("t1_not_"));
    assert_eq!(note.note_type.as_deref(), Some("note"));

    // READ
    let fetched: Option<Note> = client
        .get_one(EntityType::Notes, note.id.as_str())
        .await
        .expect("Failed to get note");

    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.id.as_str(), note.id.as_str());
    println!("Read note: {}", fetched.id.as_str());

    // UPDATE
    let updated: Note = client
        .update(
            EntityType::Notes,
            note.id.as_str(),
            &json!({"note": "Updated integration test note"}),
        )
        .await
        .expect("Failed to update note");

    assert_eq!(
        updated.note.as_deref(),
        Some("Updated integration test note")
    );
    println!("Updated note: {:?}", updated.note);

    // SEARCH notes by entity
    let search = SearchBuilder::new().field("entity", &entity_id).build();

    let notes: Vec<Note> = client
        .search(EntityType::Notes, &search)
        .await
        .expect("Failed to search notes");

    println!("Found {} notes for entity {}", notes.len(), entity_id);
    assert!(
        notes.iter().any(|n| n.id.as_str() == note.id.as_str()),
        "Should find our note in search results"
    );

    // DELETE (deactivate)
    let deactivated: Note = client
        .update(EntityType::Notes, note.id.as_str(), &json!({"inactive": 1}))
        .await
        .expect("Failed to deactivate note");

    assert!(deactivated.inactive);
    println!("Deactivated note: {}", deactivated.id.as_str());

    println!("\n=== NOTE CRUD TEST COMPLETE ===\n");
}

/// Test Alert, AlertAction, and AlertTrigger CRUD operations.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources"]
async fn test_alert_crud() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("\n=== ALERT CRUD TEST ===\n");

    // Get a login ID from an existing alert to use for our test
    let existing_alerts: Vec<Alert> = client
        .get_all(EntityType::Alerts)
        .await
        .expect("Failed to get alerts");

    let login_id = existing_alerts
        .iter()
        .find_map(|a| a.login.as_ref().map(|l| l.as_str().to_string()))
        .expect("Need an existing alert with a login to run this test");

    println!("Using login: {} for alert", login_id);

    // CREATE Alert
    let alert: Alert = client
        .create(
            EntityType::Alerts,
            &json!({
                "forlogin": &login_id,
                "name": format!("Test Alert {}", timestamp),
                "description": "Integration test alert"
            }),
        )
        .await
        .expect("Failed to create alert");

    println!("Created alert: {}", alert.id.as_str());
    println!("  Name: {:?}", alert.name);
    println!("  Description: {:?}", alert.description);

    assert!(alert.id.as_str().starts_with("t1_alt_"));

    // CREATE AlertAction (web webhook)
    let action: AlertAction = client
        .create(
            EntityType::AlertActions,
            &json!({
                "alert": alert.id.as_str(),
                "type": "web",
                "value": "https://webhook.example.com/test",
                "options": "JSON",
                "retries": 3
            }),
        )
        .await
        .expect("Failed to create alert action");

    println!("Created alert action: {}", action.id.as_str());
    println!("  Type: {:?}", action.action_type);
    println!("  Value: {:?}", action.value);

    assert!(action.id.as_str().starts_with("t1_ala_"));

    // CREATE AlertTrigger
    let trigger: AlertTrigger = client
        .create(
            EntityType::AlertTriggers,
            &json!({
                "alert": alert.id.as_str(),
                "event": "txn.created",
                "resource": 16,
                "name": format!("Test Trigger {}", timestamp),
                "description": "Triggers on transaction creation"
            }),
        )
        .await
        .expect("Failed to create alert trigger");

    println!("Created alert trigger: {}", trigger.id.as_str());
    println!("  Event: {:?}", trigger.event);
    println!("  Name: {:?}", trigger.name);

    assert!(trigger.id.as_str().starts_with("t1_alr_"));

    // READ all three
    let fetched_alert: Option<Alert> = client
        .get_one(EntityType::Alerts, alert.id.as_str())
        .await
        .expect("Failed to get alert");
    assert!(fetched_alert.is_some());
    println!("Read alert: {}", fetched_alert.unwrap().id.as_str());

    let fetched_action: Option<AlertAction> = client
        .get_one(EntityType::AlertActions, action.id.as_str())
        .await
        .expect("Failed to get alert action");
    assert!(fetched_action.is_some());
    println!("Read alert action: {}", fetched_action.unwrap().id.as_str());

    let fetched_trigger: Option<AlertTrigger> = client
        .get_one(EntityType::AlertTriggers, trigger.id.as_str())
        .await
        .expect("Failed to get alert trigger");
    assert!(fetched_trigger.is_some());
    println!(
        "Read alert trigger: {}",
        fetched_trigger.unwrap().id.as_str()
    );

    // UPDATE Alert
    let updated_alert: Alert = client
        .update(
            EntityType::Alerts,
            alert.id.as_str(),
            &json!({"description": "Updated alert description"}),
        )
        .await
        .expect("Failed to update alert");
    assert_eq!(
        updated_alert.description.as_deref(),
        Some("Updated alert description")
    );
    println!(
        "Updated alert description: {:?}",
        updated_alert.description
    );

    // UPDATE AlertAction
    let updated_action: AlertAction = client
        .update(
            EntityType::AlertActions,
            action.id.as_str(),
            &json!({"value": "https://webhook.example.com/updated"}),
        )
        .await
        .expect("Failed to update alert action");
    assert_eq!(
        updated_action.value.as_deref(),
        Some("https://webhook.example.com/updated")
    );
    println!("Updated action value: {:?}", updated_action.value);

    // UPDATE AlertTrigger
    let updated_trigger: AlertTrigger = client
        .update(
            EntityType::AlertTriggers,
            trigger.id.as_str(),
            &json!({"description": "Updated trigger description"}),
        )
        .await
        .expect("Failed to update alert trigger");
    assert_eq!(
        updated_trigger.description.as_deref(),
        Some("Updated trigger description")
    );
    println!(
        "Updated trigger description: {:?}",
        updated_trigger.description
    );

    // SEARCH alerts
    let search = SearchBuilder::new()
        .field("name[contains]", "Test Alert")
        .build();

    let alerts: Vec<Alert> = client
        .search(EntityType::Alerts, &search)
        .await
        .expect("Failed to search alerts");

    println!("Found {} alerts matching 'Test Alert'", alerts.len());

    // DELETE (deactivate) in reverse order: trigger -> action -> alert
    let _ = client
        .update::<_, AlertTrigger>(
            EntityType::AlertTriggers,
            trigger.id.as_str(),
            &json!({"inactive": 1}),
        )
        .await
        .expect("Failed to deactivate trigger");
    println!("Deactivated trigger: {}", trigger.id.as_str());

    let _ = client
        .update::<_, AlertAction>(
            EntityType::AlertActions,
            action.id.as_str(),
            &json!({"inactive": 1}),
        )
        .await
        .expect("Failed to deactivate action");
    println!("Deactivated action: {}", action.id.as_str());

    let deactivated_alert: Alert = client
        .update(
            EntityType::Alerts,
            alert.id.as_str(),
            &json!({"inactive": 1}),
        )
        .await
        .expect("Failed to deactivate alert");
    assert!(deactivated_alert.inactive);
    println!("Deactivated alert: {}", deactivated_alert.id.as_str());

    println!("\n=== ALERT CRUD TEST COMPLETE ===\n");
}
