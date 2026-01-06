//! Payrix entity types for API endpoints.

use std::fmt;

/// All Payrix API entity types.
///
/// These correspond to the API endpoints, e.g., `EntityType::Customers` maps to `/customers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType {
    /// Bank accounts
    Accounts,
    /// Customers (payment contacts)
    Customers,
    /// Business entities
    Entities,
    /// Fund balances
    Funds,
    /// Team members
    Members,
    /// Merchants
    Merchants,
    /// Organizations
    Orgs,
    /// Payouts
    Payouts,
    /// Subscription plans
    Plans,
    /// Subscriptions
    Subscriptions,
    /// Subscription tokens (links between subscriptions and payment tokens)
    SubscriptionTokens,
    /// Team logins
    TeamLogins,
    /// Payment tokens
    Tokens,
    /// Transactions
    Txns,
    /// Entity reserves
    EntityReserves,
    /// Fee rules
    FeeRules,
    /// Fees
    Fees,
    /// Organization entities
    OrgEntities,
    /// Reserve entries
    ReserveEntries,
    /// Reserves
    Reserves,
    /// Vendors
    Vendors,
    /// Account verifications
    AccountVerifications,
    /// Adjustments
    Adjustments,
    /// Batches
    Batches,
    /// Chargebacks
    Chargebacks,
    /// Chargeback messages
    ChargebackMessages,
    /// Chargeback documents
    ChargebackDocuments,
    /// Chargeback message results
    ChargebackMessageResults,
    /// Chargeback statuses
    ChargebackStatuses,
    /// Contacts
    Contacts,
    /// Disbursements
    Disbursements,
    /// Disbursement entries
    DisbursementEntries,
    /// Entries (ledger)
    Entries,
    /// Pending entries
    PendingEntries,
    /// Refunds
    Refunds,
    /// Alerts
    Alerts,
    /// Alert actions
    AlertActions,
    /// Alert triggers
    AlertTriggers,
    /// Logins
    Logins,
    /// Notes
    Notes,
    /// Note documents
    NoteDocuments,
}

impl EntityType {
    /// Get the API path segment for this entity type.
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::Accounts => "accounts",
            EntityType::Customers => "customers",
            EntityType::Entities => "entities",
            EntityType::Funds => "funds",
            EntityType::Members => "members",
            EntityType::Merchants => "merchants",
            EntityType::Orgs => "orgs",
            EntityType::Payouts => "payouts",
            EntityType::Plans => "plans",
            EntityType::Subscriptions => "subscriptions",
            EntityType::SubscriptionTokens => "subscriptionTokens",
            EntityType::TeamLogins => "team_logins",
            EntityType::Tokens => "tokens",
            EntityType::Txns => "txns",
            EntityType::EntityReserves => "entityReserves",
            EntityType::FeeRules => "feeRules",
            EntityType::Fees => "fees",
            EntityType::OrgEntities => "orgEntities",
            EntityType::ReserveEntries => "reserveEntries",
            EntityType::Reserves => "reserves",
            EntityType::Vendors => "vendors",
            EntityType::AccountVerifications => "accountVerifications",
            EntityType::Adjustments => "adjustments",
            EntityType::Batches => "batches",
            EntityType::Chargebacks => "chargebacks",
            EntityType::ChargebackMessages => "chargebackMessages",
            EntityType::ChargebackDocuments => "chargebackDocuments",
            EntityType::ChargebackMessageResults => "chargebackMessageResults",
            EntityType::ChargebackStatuses => "chargebackStatuses",
            EntityType::Contacts => "contacts",
            EntityType::Disbursements => "disbursements",
            EntityType::DisbursementEntries => "disbursementEntries",
            EntityType::Entries => "entries",
            EntityType::PendingEntries => "pendingEntries",
            EntityType::Refunds => "refunds",
            EntityType::Alerts => "alerts",
            EntityType::AlertActions => "alertActions",
            EntityType::AlertTriggers => "alertTriggers",
            EntityType::Logins => "logins",
            EntityType::Notes => "notes",
            EntityType::NoteDocuments => "noteDocuments",
        }
    }
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
