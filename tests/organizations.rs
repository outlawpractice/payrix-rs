//! Organization, Contact, and Vendor integration tests.

mod common;

use common::{create_client, init_logging};
use payrix::{Contact, EntityType, Org, OrgEntity, Vendor};

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_orgs() {
    init_logging();
    let client = create_client();

    let orgs: Vec<Org> = client.get_all(EntityType::Orgs).await.unwrap();

    println!("Found {} orgs", orgs.len());
    for org in orgs.iter().take(5) {
        println!("  Org: {} - name: {:?}", org.id.as_str(), org.name);
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_contacts() {
    init_logging();
    let client = create_client();

    let contacts: Vec<Contact> = client.get_all(EntityType::Contacts).await.unwrap();

    println!("Found {} contacts", contacts.len());
    for contact in contacts.iter().take(5) {
        println!(
            "  Contact: {} - first: {:?}, last: {:?}",
            contact.id.as_str(),
            contact.first,
            contact.last
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_org_entities() {
    init_logging();
    let client = create_client();

    let org_entities: Vec<OrgEntity> = client.get_all(EntityType::OrgEntities).await.unwrap();

    println!("Found {} org entities", org_entities.len());
    for oe in org_entities.iter().take(5) {
        println!(
            "  OrgEntity: {} - org: {:?}, entity: {:?}",
            oe.id.as_str(),
            oe.org,
            oe.entity
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_vendors() {
    init_logging();
    let client = create_client();

    let vendors: Vec<Vendor> = client.get_all(EntityType::Vendors).await.unwrap();

    println!("Found {} vendors", vendors.len());
    for vendor in vendors.iter().take(5) {
        println!(
            "  Vendor: {} - entity: {:?}, division: {:?}",
            vendor.id.as_str(),
            vendor.entity,
            vendor.division
        );
    }
}
