# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-XX-XX

### Added

- Initial release
- Full async/await API client with Tokio
- Built-in rate limiting with sliding window
- Automatic retry with exponential backoff for 429 responses
- Comprehensive type coverage for Payrix API:
  - Core: Merchants, Entities, Customers, Tokens, Transactions
  - Banking: Accounts, AccountVerifications, Funds, Disbursements
  - Billing: Subscriptions, Plans, Fees, FeeRules
  - Operations: Batches, Payouts, Reserves, Adjustments
  - Disputes: Chargebacks, ChargebackMessages, ChargebackDocuments
  - Admin: Orgs, TeamLogins, Members, Contacts, Vendors
- Search helpers with `SearchBuilder` for complex queries
- Error types with domain-specific variants
- Optional SQLx support for database storage (`sqlx` feature)

[Unreleased]: https://github.com/outlawpractice/payrix-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/outlawpractice/payrix-rs/releases/tag/v0.1.0
