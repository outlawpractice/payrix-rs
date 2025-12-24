//! Types for the Payrix API.
//!
//! This module contains all the data structures used to interact with
//! the Payrix payment processing API.
//!
//! # Feature Flags
//!
//! Some types are only available when specific feature flags are enabled:
//!
//! - `financial` - Includes financial/reporting types (Settlement, Statement)
//! - `terminal` - Includes terminal/physical device types (Terminal)
//! - `full` - Includes all optional types

// =============================================================================
// CORE TYPES (always included)
// =============================================================================
mod account;
mod account_verification;
mod adjustment;
mod alert;
mod batch;
mod chargeback;
mod common;
mod contact;
mod customer;
mod disbursement;
mod disbursement_entry;
mod division;
mod entity;
mod entity_reserve;
mod entry;
mod fee;
mod fee_rule;
mod fund;
mod hold;
mod login;
mod member;
mod merchant;
mod note;
mod org;
mod org_entity;
mod partition;
mod payment;
mod payout;
mod plan;
mod refund;
mod reserve;
mod reserve_entry;
mod subscription;
mod team_login;
mod token;
mod transaction;
mod vendor;

// Expanded types for API responses with expand[] query parameters
mod expanded;

pub use account::*;
pub use account_verification::*;
pub use adjustment::*;
pub use alert::*;
pub use batch::*;
pub use chargeback::*;
pub use common::*;
pub use contact::*;
pub use customer::*;
pub use disbursement::*;
pub use disbursement_entry::*;
pub use division::*;
pub use entity::*;
pub use entity_reserve::*;
pub use entry::*;
pub use fee::*;
pub use fee_rule::*;
pub use fund::*;
pub use hold::*;
pub use login::*;
pub use member::*;
pub use merchant::*;
pub use note::*;
pub use org::*;
pub use org_entity::*;
pub use partition::*;
pub use payment::*;
pub use payout::*;
pub use plan::*;
pub use refund::*;
pub use reserve::*;
pub use reserve_entry::*;
pub use subscription::*;
pub use team_login::*;
pub use token::*;
pub use transaction::*;
pub use vendor::*;

// Expanded types
pub use expanded::*;

// =============================================================================
// FINANCIAL/REPORTING TYPES (requires 'financial' feature)
// =============================================================================
#[cfg(feature = "financial")]
mod settlement;
#[cfg(feature = "financial")]
mod statement;

#[cfg(feature = "financial")]
pub use settlement::*;
#[cfg(feature = "financial")]
pub use statement::*;

// =============================================================================
// TERMINAL/PHYSICAL DEVICE TYPES (requires 'terminal' feature)
// =============================================================================
#[cfg(feature = "terminal")]
mod terminal;

#[cfg(feature = "terminal")]
pub use terminal::*;
