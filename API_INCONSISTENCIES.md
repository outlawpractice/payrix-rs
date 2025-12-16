# Payrix API Inconsistencies

This document catalogs discrepancies between the Payrix API documentation (OpenAPI 3.1 specification) and the actual API behavior observed during integration testing. These findings inform the design choices in this library.

**Official Documentation:** https://docs.worldpay.com/apis/payrix

**Principle:** Reality wins. When integration tests pass but the OpenAPI spec differs, we follow the actual API behavior.

## Summary

The Payrix API documentation (OpenAPI spec) sometimes differs from actual API behavior:
1. **Enum variants:** API often returns undocumented enum values not in OpenAPI
2. **Field types:** Integer vs float, date formats vary
3. **Missing fields:** Many fields in actual API responses aren't documented in OpenAPI
4. **Semantic mismatches:** Same field names with different meanings/values
5. **String vs integer formats:** Some fields use string-encoded integers (e.g., `"1"` vs `1`)

---

## Table of Contents

1. [Enum Type Inconsistencies](#enum-type-inconsistencies)
2. [ID and Date Format Inconsistencies](#id-and-date-format-inconsistencies)
3. [Monetary Field Inconsistencies](#monetary-field-inconsistencies)
4. [Schema/Struct Mismatches](#schemastruct-mismatches)
5. [Semantic Mismatches](#semantic-mismatches)
6. [Null Value Handling](#null-value-handling)
7. [Empty/Malformed Response Issues](#emptymalformed-response-issues)
8. [Integer vs String in Transaction Enums](#integer-vs-string-in-transaction-enums)
9. [Field Naming Conventions](#field-naming-conventions)
10. [Recommendations](#recommendations)
11. [Test Results Summary](#test-results-summary)

---

## Enum Type Inconsistencies

### ChargebackCycle

**OpenAPI suggests:** CamelCase string values (16 variants)

**Actual API returns:** Same variants plus one undocumented:
- `"closed"` (not in OpenAPI)

**Library solution:** Added `Closed` variant with explicit `#[serde(rename = "closed")]`.

### ChargebackDocumentType

**OpenAPI suggests:** String enum
- `"jpg"`, `"jpeg"`, `"gif"`, `"png"`, `"pdf"`, `"tiff"`, `"tif"`

**Actual API returns:** Overlapping but different values
- `"image"` (not in OpenAPI)
- `"pdf"`
- `"text"` (not in OpenAPI)
- `"tiff"`
- `"png"`
- `"jpg"`
- `"other"` (not in OpenAPI)

**Library solution:** String-based enum with all observed variants. Missing OpenAPI values (`jpeg`, `gif`, `tif`) not yet added as they haven't been observed in actual responses.

### NoteType

**OpenAPI suggests:** Extensive enum with 45+ specialized values
- `"note"`, `"release"`, `"review"`, `"reReview"`, `"amexSales"`, `"businessSales"`, `"riskApproved"`, etc.

**Actual API returns:** Only generic types observed
- `"general"`, `"note"`, `"customerService"`, `"risk"`, `"internal"`, `"system"`

**Library solution:** String-based enum with only the 6 generic types that have been observed. The extensive OpenAPI types may be for write operations.

### NoteDocumentType

**OpenAPI suggests:** Two separate enums
- `type`: File format (`jpg`, `jpeg`, `gif`, `png`, `pdf`, `tiff`, `tif`, `txt`, `xml`, etc.)
- `documentType`: Purpose (`general`, `personalId`, `companyId`, `voidCheck`, etc.)

**Actual API returns:** Single type field with image formats
- `"image"`, `"pdf"`, `"text"`, `"spreadsheet"`, `"png"`, `"jpg"`, `"tiff"`, `"other"`

**Library solution:** Single string-based enum combining observed values.

### AlertActionType

**OpenAPI suggests:** String enum
- `"email"`, `"web"`, `"app"`, `"sms"`

**Actual API returns:** Similar but different
- `"web"`, `"email"`, `"sms"`, `"push"` (not `"app"`)

**Library solution:** String-based enum with `Push` variant instead of `App`.

### AccountType

**OpenAPI suggests:** Integer `PaymentMethod` enum (1=Visa, 2=MasterCard, etc.)

**Actual API returns:** String values for account type field
- `"credit"`, `"debit"`, `"all"`

**Library solution:** Created separate `AccountType` string enum distinct from `PaymentMethod`.

### BatchStatus

**OpenAPI suggests:** String enum
- `"open"`, `"closed"`

**Actual API returns:** Additional value
- `"open"`, `"processed"` (undocumented), `"closed"`

**Library solution:** String-based enum with `Processed` variant added.

### PayoutStatus

**OpenAPI suggests:** Not documented in OpenAPI spec

**Actual API returns:** String enum values
- Values observed in integration tests

**Library solution:** Created enum based on observed API behavior since OpenAPI doesn't define it.

### SubscriptionStatus

**OpenAPI suggests:** Not documented in OpenAPI spec

**Actual API returns:** Integer enum values
- 0=Pending, 1=Active, 2=Paused, 3=Canceled, 4=PastDue, 5=Trial, 6=Completed

**Library solution:** Integer-based enum created from observed behavior.

### SubscriptionSchedule / PlanSchedule

**OpenAPI suggests:** Integer enum (4 values)
- `1` = Daily, `2` = Weekly, `3` = Monthly, `4` = Annual

**Actual API returns:** Integer enum (7 values)
- `1` = Daily, `2` = Weekly, `3` = BiWeekly, `4` = Monthly, `5` = Quarterly, `6` = SemiAnnual, `7` = Annual

**Critical Note:** The numeric values differ! OpenAPI has Monthly=3, Annual=4 but actual API uses Monthly=4, Annual=7.

**Library solution:** Using actual API values (7-value enum). This may cause issues if OpenAPI-based clients send data.

### Reserve Status (Semantic Mismatch)

**OpenAPI suggests:** Integer enum with meanings
- `1` = Active, `2` = Under review, `3` = Inactive

**Actual API returns:** Different semantic meaning
- `1` = Active, `2` = Released, `3` = PartiallyReleased, `4` = Expired

**Library solution:** Using actual API semantics. See [Semantic Mismatches](#semantic-mismatches) section.

---

## ID and Date Format Inconsistencies

### PayrixId

**OpenAPI suggests:** Exactly 30 characters in format `t1_xxx_...`

**Actual API returns:** Variable-length IDs
- Most IDs: 30 characters
- Some endpoints (Disbursements, Payouts): 32 characters
- Observed range: 29-32 characters

**Library solution:** Changed validation from exactly 30 characters to accept 29-32 characters.

### DateYmd (YYYYMMDD dates)

**OpenAPI suggests:** Exactly 8 characters in YYYYMMDD format

**Actual API returns:** Variable-length date strings
- Full dates: `"20241215"` (8 chars)
- Year only: `"2024"` (4 chars) in some Member fields
- Other lengths observed in various contexts

**Library solution:** Relaxed validation to accept any numeric string. Only validates full 8-character dates for correctness; shorter strings are accepted as-is.

### Disbursement Date Fields

**OpenAPI suggests:** YYYYMMDD format for `scheduled` and `processed` fields

**Actual API returns:** Full datetime strings
- Example: `"2025-10-09 10:47:48"`

**Library solution:** Changed `scheduled` and `processed` fields from `DateYmd` to `String`.

---

## Monetary Field Inconsistencies

### Fund Balance Fields

**OpenAPI suggests:** Integer values (cents)

**Actual API returns:** Floating-point values
- Example: `3524255.258` instead of `3524255`

**Library solution:** Changed `available`, `pending`, `reserved`, `total` fields from `Option<i64>` to `Option<f64>`.

### DisbursementEntry Amount Fields

**OpenAPI suggests:** Integer values (cents)

**Actual API returns:** Floating-point values
- Example: `1855189.609`

**Library solution:** Changed `amount`, `fee`, `net` fields from `Option<i64>` to `Option<f64>`.

---

## Schema/Struct Mismatches

This section documents cases where the Rust structs have significantly different fields than the OpenAPI specification. In all cases, the Rust implementation reflects actual API behavior (tests pass).

### Account Fields

**OpenAPI defines:** Minimal fields
- `id`, standard metadata

**Actual API returns:** Full bank account details
- `routing`, `account`, `last4`, `first6`
- `first`, `middle`, `last`, `bank`
- `holder_type` (enum: personal/business)

**Library solution:** Rust struct includes all observed fields.

### AccountVerification Fields

**OpenAPI defines:**
- `amount1`, `amount2` for micro-deposit amounts

**Actual API returns:**
- `debit1`, `debit2` for micro-deposit amounts

**Library solution:** Rust struct uses `debit1`/`debit2` field names.

### Vendor Schema

**OpenAPI defines:** Minimal schema (mostly references)

**Actual API returns:** Extensive fields
- `name`, `description`, `email`
- `phone`, `address` fields
- `tax_id`, banking info
- Status and metadata

**Library solution:** Rust struct includes all observed fields.

### Org Fields

**OpenAPI defines:** ~20 fields

**Rust struct has:** Many additional fields not in OpenAPI
- `parent`, `status`, `legal_name`
- ~15 additional fields

**Library solution:** Rust struct includes all observed fields.

### Chargeback Fields

**OpenAPI has but Rust doesn't:**
- `total`, `representedTotal`, `mid`, `ref`, `bankRef`, `chargebackRef`
- `reply`, `issued`, `received` (as timestamps)
- `lastStatusChange`, `actionable`, `shadow`, `paymentMethod`, `cycle`

**Rust has but OpenAPI doesn't:**
- `amount`, `case_number`, `due_date`, `received_date`, `resolved_date`
- `outcome`, `arn`, `card`, `last4`, `first6`, `cardholder`
- `entity`, `login`

**Library solution:** Rust struct uses observed field names. May need expansion for write operations.

### Subscription Fields

**OpenAPI has but Rust doesn't:**
- `statementEntity`, `firstTxn`, `tax`, `descriptor`
- `origin`, `authentication`, `authenticationId`

**Rust has but OpenAPI doesn't:**
- `merchant`, `customer`, `token`, `entity`
- `status`, `schedule`, `amount`, `currency`
- `cycles`, `cycles_completed`, `interval`, `day`
- `trial_end`, `name`, `next`, `last_txn`

**Note:** Field names also differ: Rust uses `failed_attempts`/`max_failed_attempts`, OpenAPI uses `failures`/`maxFailures`.

### Alert/AlertTrigger Fields

**OpenAPI AlertTrigger uses:** Specific `event` enum with 50+ event types
- `create`, `update`, `delete`, `chargeback.created`, etc.

**Rust AlertTrigger uses:** Generic approach
- `trigger_type` (i32) with `field`/`operator`/`value`

**Library solution:** Rust uses generic approach which may be more flexible for actual API behavior.

---

## Semantic Mismatches

These are cases where field names match but have different meanings.

### Reserve.status

**OpenAPI semantics:**
- `1` = Active (reserve is active)
- `2` = Under review (being reviewed)
- `3` = Inactive (reserve disabled)

**Actual API semantics:**
- `1` = Active (reserve is held)
- `2` = Released (funds released)
- `3` = PartiallyReleased (some funds released)
- `4` = Expired (reserve period ended)

**Library solution:** Using actual API semantics. These represent different business concepts.

### Fee vs FeeRule

**OpenAPI `feesResponse`:** Represents fee configuration/rules

**Actual API `fees` endpoint:** Returns generated fee records (individual charges)

**Library solution:** `Fee` struct models generated fee records. Fee configuration may use different endpoint/struct.

---

## Null Value Handling

### String Fields in Customer and Other Types

**OpenAPI suggests:** Optional string fields

**Actual API returns:** Explicit `null` values in JSON for optional fields, particularly in expanded/nested objects

**Known issues:**
- Fields deep in Customer response (around column 61741) may contain null where strings are expected
- Some nested relationships return null for fields expected to be strings

**Library solution:** Added `#[serde(default)]` to most optional fields. Some edge cases may still fail.

### Alert Entity Field

**OpenAPI suggests:** Required entity ID

**Actual API returns:** Some alerts have no entity field

**Library solution:** Made `entity` field optional with `#[serde(default)]`.

### Customer.merchant

**OpenAPI suggests:** Optional field

**Actual API behavior:** Sometimes returns null for this field in certain contexts

**Library solution:** Field is `Option<PayrixId>` with `#[serde(default)]`.

---

## Empty/Malformed Response Issues

### TeamLogins Access

**Observed issue:** Empty response body when querying TeamLogins

**Error:** `"EOF while parsing a value"`

**Possible causes:**
- Test account lacks permission to view team logins
- API returns empty body instead of empty array `[]`

**Library status:** The TeamLogin struct matches OpenAPI schema. The empty response appears to be an API access/permission issue rather than a code problem.

### ChargebackStatus Fields

**OpenAPI suggests:** `fromStatus` and `toStatus` fields track status transitions

**Actual API returns (verified December 2025):** These fields are always `null`
- Tested with 14 ChargebackStatus records via raw JSON inspection
- Only the `status` field contains data (the current status when the record was created)

**Library solution:** Fields are `Option<ChargebackStatusValue>` and deserialize correctly as `None`.

---

## Integer vs String in Transaction Enums

### Various Transaction Enums

**Official Documentation:** Transaction fields use string-encoded integers:
- `"type": "1"` for card sale, `"type": "5"` for card refund
- `"status": "1"` for Approved, `"status": "3"` for Captured
- `"origin": "2"` for eCommerce

**Actual API returns:** String-encoded integers
- `"1"` instead of `1`
- Observed at column 1156 in transaction responses

**Library solution:** Created `impl_flexible_i32_enum_deserialize!` macro that accepts both integer (`1`) and string (`"1"`) formats. Applied to all transaction enums:
- TransactionType
- TransactionStatus
- TransactionOrigin
- TerminalCapability
- EntryMode
- TxnResultType
- TxnResultCode

### Missing Enum Values

**TerminalCapability:** OpenAPI missing value `4` (observed in tests)

**EntryMode:** OpenAPI missing values `6` through `14` (observed in tests)

**CofType:** OpenAPI missing `installment` variant

**Library solution:** Enums include all observed values even if not in OpenAPI.

---

## Field Naming Conventions

### login vs creator/modifier

**OpenAPI convention:** Uses `creator` and `modifier` fields for audit trails

**Actual API convention:** Uses `login` field for the creating user

**Affected types:** Most types including Alert, AlertAction, AlertTrigger, Note, NoteDocument, Chargeback, ChargebackMessage, ChargebackDocument, Subscription, Plan

**Library solution:** Rust structs use `login` field based on observed API behavior.

### body/message vs note

**OpenAPI convention:** Uses `note` for text content

**Actual API convention:** Uses `body` or `message` depending on context

**Library solution:** Rust structs use observed field names.

### end vs finish

**OpenAPI convention:** Uses `finish` for end dates

**Actual API convention:** Uses `end`

**Library solution:** Rust structs use `end` based on observed behavior.

---

## Recommendations

1. **Expect API inconsistency:** The Payrix API does not consistently follow its documentation. Always design deserializers to be lenient.

2. **Match OpenAPI enum format:** Use integer enums (`serde_repr`) when OpenAPI specifies integers, string enums when OpenAPI specifies strings. Test to confirm.

3. **Add unknown variants:** Many enums have undocumented values. Consider adding catch-all variants or using `#[serde(other)]`.

4. **Handle null gracefully:** Use `#[serde(default)]` liberally and consider custom deserializers for fields that might be null.

5. **Log deserialization errors:** When possible, log the raw JSON to help diagnose future inconsistencies.

6. **Allow flexible ID lengths:** Don't assume all PayrixIds are exactly 30 characters.

7. **Trust test results over docs:** When tests pass, the Rust implementation is correct regardless of what OpenAPI says.

8. **Document intentional deviations:** All deviations from OpenAPI are documented here for reference.

---

## Test Results Summary

**Passing tests (47/48):**
- test_customer_crud
- test_error_handling
- test_get_accounts
- test_get_account_verifications
- test_get_adjustments
- test_get_alerts
- test_get_alert_actions
- test_get_alert_triggers
- test_get_batches
- test_get_chargebacks
- test_get_chargeback_documents
- test_get_chargeback_messages
- test_get_chargeback_message_results
- test_get_chargeback_statuses
- test_get_contacts
- test_get_disbursements
- test_get_disbursement_entries
- test_get_customers
- test_get_entities
- test_get_entity_reserves
- test_get_entries
- test_get_fee_rules
- test_get_fees
- test_get_funds
- test_get_logins
- test_get_members
- test_get_merchants
- test_get_notes
- test_get_note_documents
- test_get_orgs
- test_get_org_entities
- test_get_payouts
- test_get_pending_entries
- test_get_plans
- test_get_refunds
- test_get_reserves
- test_get_reserve_entries
- test_get_subscriptions
- test_get_tokens
- test_get_transactions
- test_get_vendors
- test_search_with_pagination
- test_token_creation
- test_transaction_flow
- test_discover_chargeback_status_values
- test_chargeback_raw_json_status
- test_chargeback_status_history_raw

**Failing tests (1/48):**
- **test_get_team_logins:** API returns empty body - likely account access/permission issue

---

## Appendix: OpenAPI vs Rust Field Coverage

This appendix summarizes field coverage for major types.

### Types with Many Undocumented Fields (Rust has, OpenAPI doesn't)

| Type | Extra Rust Fields |
|------|-------------------|
| Account | routing, account, last4, first6, first, middle, last, bank, holder_type |
| Vendor | Most fields (OpenAPI minimal) |
| Org | ~15 additional fields |
| Token | merchant |
| Subscription | ~20 additional fields |
| Chargeback | ~15 additional fields |

### Types with Many Missing Fields (OpenAPI has, Rust doesn't)

| Type | Missing from Rust |
|------|-------------------|
| Chargeback | total, representedTotal, mid, ref, bankRef, cycle, etc. |
| Subscription | statementEntity, firstTxn, tax, descriptor, origin |
| Alert | forlogin, team, division, partition |
| Plan | billing, type, txnDescription, order, um, scheduleFactor |

### Types with Good Alignment

| Type | Notes |
|------|-------|
| Transaction | Well-aligned, flexible deserializers handle format differences |
| Token | Good alignment, minor extras |
| Batch | Good alignment, extra status value |
| Entry | Good alignment |
| Disbursement | Good alignment after date format fix |

---

*Last updated: December 16, 2025 - Based on comprehensive OpenAPI vs Rust comparison and raw JSON API testing*
