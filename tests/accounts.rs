//! Account and Account Verification integration tests.

mod common;

use common::{create_client, init_logging};
use payrix::{Account, AccountVerification, EntityType};

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_accounts() {
    init_logging();
    let client = create_client();

    let accounts: Vec<Account> = client.get_all(EntityType::Accounts).await.unwrap();

    println!("Found {} accounts", accounts.len());
    for account in accounts.iter().take(5) {
        println!(
            "  Account: {} - type: {:?}, status: {:?}",
            account.id.as_str(),
            account.account_type,
            account.status
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_account_verifications() {
    init_logging();
    let client = create_client();

    let verifications: Vec<AccountVerification> = client
        .get_all(EntityType::AccountVerifications)
        .await
        .unwrap();

    println!("Found {} account verifications", verifications.len());
    for v in verifications.iter().take(5) {
        println!(
            "  AccountVerification: {} - account: {:?}, type: {:?}, debit1: {:?}",
            v.id.as_str(),
            v.account,
            v.verification_type,
            v.debit1
        );
    }
}
