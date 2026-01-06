//! Hold types for the Payrix API.
//!
//! Holds represent blocks, holds, or reserves placed on transactions or entities
//! for risk management purposes.
//!
//! **OpenAPI schema:** `holdsResponse`

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// ENUMS
// =============================================================================

/// Hold action values per OpenAPI spec.
///
/// **OpenAPI schema:** `holdAction`
///
/// Valid values:
/// - `0` - NONE
/// - `1` - Block (returns an error)
/// - `3` - Hold (will not be captured until manually released)
/// - `4` - Reserve (funds will not be released until manual review)
/// - `5` - LIMIT (block current activity)
/// - `6` - PASS (passed decision(s))
/// - `8` - POSTREVIEWONLY
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum HoldAction {
    /// No action.
    None = 0,
    /// Block the Transaction from proceeding.
    Block = 1,
    /// Hold the Transaction, will not be captured until manually released.
    Hold = 3,
    /// Reserve the Transaction, funds not released until manual review.
    Reserve = 4,
    /// Block current activity.
    Limit = 5,
    /// Passed decision(s).
    Pass = 6,
    /// Post review only.
    PostReviewOnly = 8,
}

/// Release action values per OpenAPI spec.
///
/// **OpenAPI schema:** `ReleaseAction`
///
/// Valid values:
/// - `1` - Approved
/// - `2` - Cancelled
/// - `3` - Refunded
/// - `4` - Failed
/// - `5` - Expired
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ReleaseAction {
    /// Holds released because the hold resource was approved.
    Approved = 1,
    /// Holds released because the hold resource was cancelled.
    Cancelled = 2,
    /// Holds released because the hold resource was refunded.
    Refunded = 3,
    /// Holds released because the hold resource was failed.
    Failed = 4,
    /// Expired.
    Expired = 5,
}

/// Hold source values per OpenAPI spec.
///
/// **OpenAPI schema:** `HoldSource`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HoldSource {
    /// DS model policy run.
    DsModelPolicyRun,
    /// API decision.
    ApiDecision,
    /// Policy run.
    PolicyRun,
    /// Risk alert.
    RiskAlert,
    /// Manual.
    Manual,
    /// Error.
    Error,
}

// =============================================================================
// HOLD STRUCT
// =============================================================================

/// A Payrix hold.
///
/// Holds represent blocks, holds, or reserves placed on transactions or entities
/// for risk management purposes.
///
/// **OpenAPI schema:** `holdsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Hold {
    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string (ref: creator)
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// The identifier of the Login that owns this holds resource.
    ///
    /// **OpenAPI type:** string (ref: holdsModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The identifier of the Entity associated with this Hold resource.
    ///
    /// **OpenAPI type:** string (ref: holdsModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The identifier of the Txn that is being held with this hold.
    ///
    /// **OpenAPI type:** string (ref: holdsModelTxn)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// The TerminalTxn id in which this Hold belongs to.
    ///
    /// **OpenAPI type:** string (ref: holdsModelTerminalTxn)
    #[serde(default)]
    pub terminal_txn: Option<PayrixId>,

    /// The identifier of the Account that owns this holds resource.
    ///
    /// **OpenAPI type:** string (ref: holdsModelAccount)
    #[serde(default)]
    pub account: Option<PayrixId>,

    /// If this hold was triggered through a verification, the identifier of the Verification.
    ///
    /// **OpenAPI type:** string (ref: holdsModelVerification)
    #[serde(default)]
    pub verification: Option<PayrixId>,

    /// If this hold was triggered through Payrix Integration Risk, the identifier of the VerificationRef.
    ///
    /// **OpenAPI type:** string (ref: holdsModelVerificationRef)
    #[serde(default)]
    pub verification_ref: Option<PayrixId>,

    /// The identifier of the DecisionAction associated with this Hold resource.
    ///
    /// **OpenAPI type:** string (ref: holdsModelDecisionAction)
    #[serde(default)]
    pub decision_action: Option<PayrixId>,

    /// The action taken on the referenced Txn.
    ///
    /// **OpenAPI type:** integer (ref: holdAction)
    #[serde(default)]
    pub action: Option<HoldAction>,

    /// If this hold was released, the timestamp for when it was released.
    ///
    /// Format: YYYY-MM-DD HH:MM:SS
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$`)
    #[serde(default)]
    pub released: Option<String>,

    /// If this hold was reviewed, the timestamp for when it was reviewed.
    ///
    /// Format: YYYY-MM-DD HH:MM:SS
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$`)
    #[serde(default)]
    pub reviewed: Option<String>,

    /// Whether this resource is marked as inactive.
    ///
    /// - `0` - Active
    /// - `1` - Inactive
    ///
    /// **OpenAPI type:** integer (ref: Inactive)
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    ///
    /// **OpenAPI type:** integer (ref: Frozen)
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    /// The final action taken on the hold while it's been released.
    ///
    /// **OpenAPI type:** integer (ref: ReleaseAction)
    #[serde(default)]
    pub release_action: Option<ReleaseAction>,

    /// The timestamp when hold was considered to be delaying funding.
    ///
    /// Format: YYYY-MM-DD HH:MM:SS
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$`)
    #[serde(default)]
    pub delayed_funding_start_date: Option<String>,

    /// The timestamp when hold stopped delaying funding.
    ///
    /// This might be due to the hold being released, cancelled or funded.
    /// Format: YYYY-MM-DD HH:MM:SS
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$`)
    #[serde(default)]
    pub delayed_funding_end_date: Option<String>,

    /// The person who reviewed the hold applied.
    ///
    /// **OpenAPI type:** string (ref: holdsModelAnalyst)
    #[serde(default)]
    pub analyst: Option<PayrixId>,

    /// The timestamp for the most recent update of the analyst field.
    ///
    /// Format: YYYY-MM-DD HH:MM:SS
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$`)
    #[serde(default)]
    pub claimed: Option<String>,

    /// Field created to know the reason for why a hold was created.
    ///
    /// **OpenAPI type:** string (ref: HoldSource)
    #[serde(default)]
    pub hold_source: Option<HoldSource>,

    /// Reason id for why a hold was created.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub hold_source_id: Option<String>,

    /// Additional details about the hold source.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub hold_source_details: Option<String>,

    /// ID of the division associated with this hold.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub division: Option<PayrixId>,

    /// Reserve associated with this hold.
    ///
    /// **OpenAPI type:** object (ref: reservesResponse)
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub reserve: Option<serde_json::Value>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hold_deserialize_full() {
        let json = r#"{
            "id": "t1_hld_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "entity": "t1_ent_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "action": 3,
            "released": "2024-01-05 12:00:00",
            "reviewed": "2024-01-05 11:00:00",
            "inactive": 0,
            "frozen": 0,
            "releaseAction": 1,
            "analyst": "t1_lgn_12345678901234567890126",
            "holdSource": "POLICY_RUN",
            "division": "t1_div_12345678901234567890123"
        }"#;

        let hold: Hold = serde_json::from_str(json).unwrap();
        assert_eq!(hold.id.as_str(), "t1_hld_12345678901234567890123");
        assert_eq!(hold.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(
            hold.entity.as_ref().map(|e| e.as_str()),
            Some("t1_ent_12345678901234567890123")
        );
        assert_eq!(hold.action, Some(HoldAction::Hold));
        assert_eq!(hold.released, Some("2024-01-05 12:00:00".to_string()));
        assert_eq!(hold.release_action, Some(ReleaseAction::Approved));
        assert_eq!(hold.hold_source, Some(HoldSource::PolicyRun));
        assert!(!hold.inactive);
        assert!(!hold.frozen);
    }

    #[test]
    fn hold_deserialize_minimal() {
        let json = r#"{"id": "t1_hld_12345678901234567890123"}"#;

        let hold: Hold = serde_json::from_str(json).unwrap();
        assert_eq!(hold.id.as_str(), "t1_hld_12345678901234567890123");
        assert!(hold.created.is_none());
        assert!(hold.entity.is_none());
        assert!(hold.action.is_none());
        assert!(!hold.inactive);
    }

    #[test]
    fn hold_action_values() {
        let test_cases = vec![
            (0, HoldAction::None),
            (1, HoldAction::Block),
            (3, HoldAction::Hold),
            (4, HoldAction::Reserve),
            (5, HoldAction::Limit),
            (6, HoldAction::Pass),
        ];

        for (val, expected) in test_cases {
            let json = format!(
                r#"{{"id": "t1_hld_12345678901234567890123", "action": {}}}"#,
                val
            );
            let hold: Hold = serde_json::from_str(&json).unwrap();
            assert_eq!(hold.action, Some(expected));
        }
    }

    #[test]
    fn hold_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_hld_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "action": 3
        }"#;

        let hold: Hold = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&hold).unwrap();
        let deserialized: Hold = serde_json::from_str(&serialized).unwrap();
        assert_eq!(hold.id, deserialized.id);
        assert_eq!(hold.entity, deserialized.entity);
        assert_eq!(hold.action, deserialized.action);
    }
}
