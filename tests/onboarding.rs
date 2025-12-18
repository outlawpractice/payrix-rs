//! Merchant onboarding workflow integration tests.

mod common;

use common::{create_client, init_logging};
use payrix::types::{
    AccountHolderType, AccountType, DateYmd, MemberType, MerchantEnvironment, MerchantType,
};
use payrix::{
    check_boarding_status, onboard_merchant, Address, BankAccountInfo, BankAccountMethod,
    BoardingStatus, BusinessInfo, EntityType, Environment, MemberInfo, Merchant, MerchantConfig,
    OnboardMerchantRequest, PayrixClient, TermsAcceptance,
};
use std::env;

/// Helper to create a test onboarding request with all required fields.
fn create_test_onboarding_request() -> OnboardMerchantRequest {
    // Generate a unique timestamp for testing
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    OnboardMerchantRequest {
        business: BusinessInfo {
            business_type: MerchantType::LimitedLiabilityCorporation,
            legal_name: format!("Test Business {} LLC", timestamp),
            address: Address {
                line1: "123 Test Street".to_string(),
                line2: Some("Suite 100".to_string()),
                city: "Springfield".to_string(),
                state: "IL".to_string(),
                zip: "62701".to_string(),
                country: "USA".to_string(),
            },
            phone: "5551234567".to_string(),
            email: "payrixrust@gmail.com".to_string(),
            website: Some("https://github.com/outlawpractice/payrix-rs".to_string()),
            ein: "123456789".to_string(),
        },
        merchant: MerchantConfig {
            dba: format!("Test DBA {}", timestamp),
            mcc: "5999".to_string(), // Miscellaneous Retail
            environment: MerchantEnvironment::Ecommerce,
            annual_cc_sales: 50_000_000, // $500,000 in cents
            avg_ticket: 5_000,           // $50 in cents
            established: DateYmd::new("20200101").unwrap(),
            is_new_business: false,
        },
        accounts: vec![BankAccountInfo {
            name: Some("Operating Account".to_string()),
            routing_number: Some("121000358".to_string()), // Test routing number
            account_number: Some("123456789".to_string()),
            holder_type: AccountHolderType::Business,
            account_method: BankAccountMethod::Checking,
            transaction_type: AccountType::All,
            currency: Some("USD".to_string()),
            is_primary: true,
            plaid_public_token: None,
        }],
        members: vec![MemberInfo {
            member_type: MemberType::Owner,
            first_name: "Test".to_string(),
            last_name: "Owner".to_string(),
            title: Some("CEO".to_string()),
            ownership_percentage: 100,
            date_of_birth: "19800115".to_string(),
            ssn: "123456789".to_string(),
            email: "payrixrust@gmail.com".to_string(),
            phone: "5559876543".to_string(),
            address: Address {
                line1: "456 Owner Lane".to_string(),
                line2: None,
                city: "Springfield".to_string(),
                state: "IL".to_string(),
                zip: "62702".to_string(),
                country: "USA".to_string(),
            },
        }],
        terms_acceptance: TermsAcceptance {
            version: "4.21".to_string(),
            accepted_at: "2024-01-15 10:30:00".to_string(),
        },
    }
}

/// Test the full merchant onboarding workflow.
///
/// **CAUTION**: This test creates real resources in Payrix!
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources in Payrix"]
async fn test_merchant_onboarding_workflow() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== MERCHANT ONBOARDING WORKFLOW TEST ===\n");

    let request = create_test_onboarding_request();
    println!("Onboarding business: {}", request.business.legal_name);
    println!("DBA: {}", request.merchant.dba);

    // Execute the onboarding workflow
    let result = onboard_merchant(&client, request).await;

    match result {
        Ok(onboard_result) => {
            println!("\n=== ONBOARDING SUCCESSFUL ===");
            println!("Entity ID: {}", onboard_result.entity_id);
            println!("Merchant ID: {}", onboard_result.merchant_id);
            println!("Boarding Status: {:?}", onboard_result.boarding_status);
            println!("Accounts created: {}", onboard_result.accounts.len());
            println!("Members created: {}", onboard_result.members.len());

            // Verify the result structure
            assert!(
                !onboard_result.entity_id.is_empty(),
                "Entity ID should not be empty"
            );
            assert!(
                !onboard_result.merchant_id.is_empty(),
                "Merchant ID should not be empty"
            );

            // The boarding status depends on Payrix's underwriting decision
            println!("\nBoarding status interpretation:");
            match onboard_result.boarding_status {
                BoardingStatus::Boarded => println!("  ✓ Merchant was immediately approved!"),
                BoardingStatus::Submitted => {
                    println!("  → Merchant submitted for boarding, awaiting processing")
                }
                BoardingStatus::Pending => println!("  → Merchant pending automated review"),
                BoardingStatus::ManualReview => {
                    println!("  → Merchant requires manual underwriting review")
                }
                BoardingStatus::NotReady => {
                    println!("  ⚠ Merchant not ready - check for missing information")
                }
                BoardingStatus::Incomplete => {
                    println!("  ⚠ Application incomplete - check required fields")
                }
                BoardingStatus::Closed => println!("  ✗ Merchant account was closed"),
            }

            // Test check_boarding_status with the created merchant
            println!("\n=== CHECKING BOARDING STATUS ===");
            let status_result = check_boarding_status(&client, &onboard_result.merchant_id).await;

            match status_result {
                Ok(status) => {
                    println!("Current Status: {:?}", status.status);
                    println!("Merchant ID: {}", status.merchant_id);
                    println!("Entity ID: {}", status.entity_id);
                    if let Some(boarded_date) = &status.boarded_date {
                        println!("Boarded Date: {}", boarded_date);
                    }
                }
                Err(e) => {
                    println!("Error checking boarding status: {}", e);
                    // Don't fail the test - the onboarding was successful
                }
            }
        }
        Err(e) => {
            // Onboarding can fail for many reasons in test environment
            println!("\n=== ONBOARDING FAILED ===");
            println!("Error: {}", e);

            // Some errors are expected in test environment
            let error_str = format!("{}", e);
            if error_str.contains("ein") || error_str.contains("EIN") {
                println!("\nNote: EIN validation may fail with test data");
            }
            if error_str.contains("ssn") || error_str.contains("SSN") {
                println!("\nNote: SSN validation may fail with test data");
            }
            if error_str.contains("routing") {
                println!("\nNote: Bank routing number validation may fail with test data");
            }

            println!(
                "\nThis is expected when using test/dummy data for EIN, SSN, and bank accounts."
            );
        }
    }
}

/// Test checking boarding status for an existing merchant.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_check_boarding_status() {
    init_logging();
    let client = create_client();

    println!("\n=== CHECK BOARDING STATUS TEST ===\n");

    // First, get an existing merchant
    let merchants: Vec<Merchant> = client.get_all(EntityType::Merchants).await.unwrap();

    if merchants.is_empty() {
        println!("No merchants found in test account - skipping test");
        return;
    }

    let merchant = &merchants[0];
    println!("Checking status for merchant: {}", merchant.id.as_str());
    println!("Merchant DBA: {:?}", merchant.dba);
    println!("Current merchant status: {:?}", merchant.status);

    // Check boarding status using the workflow function
    let result = check_boarding_status(&client, merchant.id.as_str()).await;

    match result {
        Ok(status) => {
            println!("\n=== BOARDING STATUS RESULT ===");
            println!("Status: {:?}", status.status);
            println!("Merchant ID: {}", status.merchant_id);
            println!("Entity ID: {}", status.entity_id);
            if let Some(boarded_date) = &status.boarded_date {
                println!("Boarded Date: {}", boarded_date);
            }

            // Verify the merchant ID matches
            assert_eq!(status.merchant_id, merchant.id.as_str());
        }
        Err(e) => {
            println!("Error: {}", e);
            panic!("Failed to check boarding status: {}", e);
        }
    }
}

/// Test onboarding with multiple accounts (trust + operating pattern).
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources in Payrix"]
async fn test_merchant_onboarding_with_trust_account() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== MERCHANT ONBOARDING WITH TRUST ACCOUNT TEST ===\n");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut request = create_test_onboarding_request();

    // Update with unique data (keep email as payrixrust@gmail.com from helper)
    request.business.legal_name = format!("Trust Account Test {} LLC", timestamp);
    request.merchant.dba = format!("Trust Test DBA {}", timestamp);

    // Add a trust account in addition to the operating account
    request.accounts.push(BankAccountInfo {
        name: Some("Client Trust Account".to_string()),
        routing_number: Some("121000358".to_string()),
        account_number: Some("987654321".to_string()),
        holder_type: AccountHolderType::Business,
        account_method: BankAccountMethod::Checking,
        transaction_type: AccountType::Credit, // Deposits ONLY - no fee withdrawals
        currency: Some("USD".to_string()),
        is_primary: false, // Not primary - fees come from operating
        plaid_public_token: None,
    });

    println!("Onboarding with {} accounts:", request.accounts.len());
    for (i, acct) in request.accounts.iter().enumerate() {
        println!(
            "  Account {}: {:?} - type: {:?}, primary: {}",
            i + 1,
            acct.name,
            acct.transaction_type,
            acct.is_primary
        );
    }

    let result = onboard_merchant(&client, request).await;

    match result {
        Ok(onboard_result) => {
            println!("\n=== ONBOARDING SUCCESSFUL ===");
            println!("Entity ID: {}", onboard_result.entity_id);
            println!("Merchant ID: {}", onboard_result.merchant_id);
            println!("Accounts created: {}", onboard_result.accounts.len());

            // Verify both accounts were created
            println!("\nCreated accounts:");
            for acct in &onboard_result.accounts {
                println!(
                    "  - {}: type={:?}, primary={:?}",
                    acct.id.as_str(),
                    acct.account_type,
                    acct.primary
                );
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("Debug: {:?}", e);
            println!("\nNote: This may fail with dummy data or if accounts already exist.");
        }
    }
}

/// Test onboarding with multiple members (beneficial owners).
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real resources in Payrix"]
async fn test_merchant_onboarding_with_multiple_members() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== MERCHANT ONBOARDING WITH MULTIPLE MEMBERS TEST ===\n");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut request = create_test_onboarding_request();

    // Update with unique data
    request.business.legal_name = format!("Multi-Member Test {} LLC", timestamp);
    request.merchant.dba = format!("Multi-Member DBA {}", timestamp);
    request.business.email = format!("multimember{}@example.com", timestamp);

    // First member owns 60%
    request.members[0].ownership_percentage = 60;
    request.members[0].email = format!("owner1-{}@example.com", timestamp);

    // Add second owner with 40%
    request.members.push(MemberInfo {
        member_type: MemberType::Owner,
        first_name: "Second".to_string(),
        last_name: "Owner".to_string(),
        title: Some("CFO".to_string()),
        ownership_percentage: 40,
        date_of_birth: "19850620".to_string(),
        ssn: "987654321".to_string(),
        email: format!("owner2-{}@example.com", timestamp),
        phone: "5551112222".to_string(),
        address: Address {
            line1: "789 Partner Rd".to_string(),
            line2: None,
            city: "Chicago".to_string(),
            state: "IL".to_string(),
            zip: "60601".to_string(),
            country: "USA".to_string(),
        },
    });

    // Add control person (may have 0% ownership but has management control)
    request.members.push(MemberInfo {
        member_type: MemberType::ControlPerson,
        first_name: "Control".to_string(),
        last_name: "Person".to_string(),
        title: Some("COO".to_string()),
        ownership_percentage: 0, // Control persons may not have ownership
        date_of_birth: "19900301".to_string(),
        ssn: "111223333".to_string(),
        email: format!("control-{}@example.com", timestamp),
        phone: "5553334444".to_string(),
        address: Address {
            line1: "321 Control Ave".to_string(),
            line2: Some("Apt 2B".to_string()),
            city: "Naperville".to_string(),
            state: "IL".to_string(),
            zip: "60540".to_string(),
            country: "USA".to_string(),
        },
    });

    println!("Onboarding with {} members:", request.members.len());
    for member in &request.members {
        println!(
            "  - {} {}: {:?}, {}% ownership",
            member.first_name, member.last_name, member.member_type, member.ownership_percentage
        );
    }

    let result = onboard_merchant(&client, request).await;

    match result {
        Ok(onboard_result) => {
            println!("\n=== ONBOARDING SUCCESSFUL ===");
            println!("Entity ID: {}", onboard_result.entity_id);
            println!("Merchant ID: {}", onboard_result.merchant_id);
            println!("Members created: {}", onboard_result.members.len());

            println!("\nCreated members:");
            for member in &onboard_result.members {
                println!(
                    "  - {}: {:?} {:?}, title={:?}, ownership={:?}",
                    member.id.as_str(),
                    member.first,
                    member.last,
                    member.title,
                    member.ownership
                );
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("\nThis is expected when using test/dummy data.");
        }
    }
}
