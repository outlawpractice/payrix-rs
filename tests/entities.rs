//! Entity, Merchant, Member, and Login integration tests.

mod common;

use common::{create_client, init_logging};
use payrix::{Entity, EntityType, Login, Member, Merchant, TeamLogin};

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_entities() {
    init_logging();
    let client = create_client();

    let entities: Vec<Entity> = client.get_all(EntityType::Entities).await.unwrap();

    assert!(!entities.is_empty(), "Should have at least one entity");
    println!("Found {} entities", entities.len());

    for entity in &entities {
        println!("  Entity: {} - {:?}", entity.id.as_str(), entity.name);
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_merchants() {
    init_logging();
    let client = create_client();

    let merchants: Vec<Merchant> = client.get_all(EntityType::Merchants).await.unwrap();

    println!("Found {} merchants", merchants.len());
    for merchant in merchants.iter().take(5) {
        println!(
            "  Merchant: {} - {:?} (status: {:?})",
            merchant.id.as_str(),
            merchant.dba,
            merchant.status
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_logins() {
    init_logging();
    let client = create_client();

    let logins: Vec<Login> = client.get_all(EntityType::Logins).await.unwrap();

    println!("Found {} logins", logins.len());
    for login in logins.iter().take(5) {
        println!("  Login: {} - email: {:?}", login.id.as_str(), login.email);
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_team_logins() {
    init_logging();
    let client = create_client();

    let team_logins: Vec<TeamLogin> = client.get_all(EntityType::TeamLogins).await.unwrap();

    println!("Found {} team logins", team_logins.len());
    for login in team_logins.iter().take(5) {
        println!(
            "  TeamLogin: {} - login: {:?}, team: {:?}, create: {}, read: {}",
            login.id.as_str(),
            login.login,
            login.team,
            login.create,
            login.read
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_members() {
    init_logging();
    let client = create_client();

    let members: Vec<Member> = client.get_all(EntityType::Members).await.unwrap();

    println!("Found {} members", members.len());
    for member in members.iter().take(5) {
        println!(
            "  Member: {} - merchant: {:?}",
            member.id.as_str(),
            member.merchant
        );
    }
}
