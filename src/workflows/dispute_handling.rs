//! Chargeback dispute handling workflow with compile-time state enforcement.
//!
//! This module provides a high-level, type-safe API for handling chargeback disputes
//! using Rust's typestate pattern to ensure valid state transitions at compile time.
//!
//! # Chargeback Lifecycle
//!
//! Chargebacks follow a specific lifecycle where certain actions are only valid
//! at certain stages:
//!
//! ```text
//!                                     ┌──────────────────────────────────────┐
//!                                     │          (cycle can repeat)          │
//!                                     ▼                                      │
//! Retrieval → First → Representment → Pre-Arbitration → Second Chargeback ──┘
//!     │         │          │               │                    │
//!     │         │          │               │               ┌────┴────┐
//!     │    ┌────┴────┐     │          ┌────┴────┐          │         │
//!     ▼    ▼         ▼     ▼          ▼         ▼          ▼         ▼
//!  (wait) Represent Accept (await)  Accept   Request    Represent Accept
//!         + Evidence Liability      Liability Arbitration + Evidence Liability
//!                                                               │
//!                                                               ▼
//!                                                          Arbitration
//!                                                               │
//!                                                     ┌─────────┼─────────┐
//!                                                     ▼         ▼         ▼
//!                                                    Won       Lost     Split
//! ```
//!
//! # Typestate Pattern
//!
//! This module uses the typestate pattern to ensure that:
//! - `represent()` is only callable in `First`, `PreArbitration`, and `SecondChargeback` states
//! - `accept_liability()` is only callable in `First`, `PreArbitration`, and `SecondChargeback` states
//! - `request_arbitration()` is only callable in `PreArbitration` state
//!
//! Attempting to call these methods in invalid states results in a **compile error**,
//! not a runtime error.
//!
//! # Key Insight: Stateless API
//!
//! Chargebacks can take weeks to months to resolve. This API is designed to be:
//! - **Stateless** - Always load fresh state from Payrix API
//! - **Event-driven** - Works with webhooks to track state changes
//! - **Refreshable** - Easy to reload state from the latest API data
//!
//! # Example
//!
//! ```no_run
//! use payrix::{PayrixClient, Environment};
//! use payrix::workflows::dispute_handling::{ChargebackDispute, ActiveDispute, Evidence};
//!
//! # async fn example() -> payrix::Result<()> {
//! let client = PayrixClient::new("api-key", Environment::Test)?;
//!
//! // Load a chargeback - runtime state becomes compile-time type
//! let dispute = ChargebackDispute::load(&client, "t1_chb_123").await?;
//!
//! match dispute {
//!     ChargebackDispute::Active(active) => match active {
//!         ActiveDispute::First(first) => {
//!             // represent() ONLY available here - won't compile elsewhere
//!             let evidence = Evidence::new("Customer received goods as described")
//!                 .with_document("receipt.pdf", vec![/* pdf bytes */], "application/pdf");
//!             let represented = first.represent(&client, evidence).await?;
//!             println!("Chargeback represented, now in representment stage");
//!         }
//!         ActiveDispute::PreArbitration(pre_arb) => {
//!             // request_arbitration() ONLY available here
//!             let arbitrating = pre_arb.request_arbitration(&client).await?;
//!             println!("Arbitration requested");
//!         }
//!         _ => {
//!             println!("No action required at this stage");
//!         }
//!     },
//!     ChargebackDispute::Terminal(terminal) => {
//!         println!("Dispute closed: {:?}", terminal.inner().status);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Evidence Requirements
//!
//! When representing a chargeback, evidence must meet Payrix requirements:
//! - Maximum 8 documents per representment
//! - Maximum 1 MB per document
//! - Maximum 8 MB total combined
//! - Supported formats: TIFF/TIF, PDF (primary); PNG, JPG, GIF (also accepted)
//! - Must submit 5 business days before the reply deadline

use std::marker::PhantomData;
use std::path::Path;

use base64::Engine;

use crate::client::PayrixClient;
use crate::entity::EntityType;
use crate::error::{Error, Result};
use crate::types::{
    Chargeback, ChargebackCycle, ChargebackDocument, ChargebackDocumentType, ChargebackMessage,
    ChargebackMessageType, ChargebackStatusValue, NewChargebackDocument, NewChargebackMessage,
    PayrixId,
};

// =============================================================================
// Evidence Constants (Payrix Requirements)
// =============================================================================

/// Maximum number of documents allowed per representment.
pub const MAX_DOCUMENTS: usize = 8;

/// Maximum size per document in bytes (1 MB).
pub const MAX_DOCUMENT_SIZE: usize = 1_048_576;

/// Maximum total size for all documents in bytes (8 MB).
pub const MAX_TOTAL_SIZE: usize = 8_388_608;

// =============================================================================
// Section 1: State Marker Types
// =============================================================================

/// Sealed trait module to prevent external implementations.
mod private {
    pub trait Sealed {}
}

/// Trait for chargeback state markers.
///
/// This trait is sealed and cannot be implemented outside this module,
/// ensuring that only the predefined states are valid.
pub trait ChargebackState: private::Sealed {
    /// Returns the human-readable name of this state.
    fn state_name() -> &'static str;
}

/// Initial retrieval stage - awaiting first chargeback.
///
/// At this stage, no actions are available. The merchant must wait
/// for the issuer to file the first chargeback.
#[derive(Debug, Clone, Copy)]
pub struct Retrieval;

impl private::Sealed for Retrieval {}
impl ChargebackState for Retrieval {
    fn state_name() -> &'static str {
        "retrieval"
    }
}

/// First chargeback stage - merchant can respond.
///
/// Available actions:
/// - [`TypedChargeback::represent`] - Submit evidence to dispute the chargeback
/// - [`TypedChargeback::accept_liability`] - Accept the chargeback
#[derive(Debug, Clone, Copy)]
pub struct First;

impl private::Sealed for First {}
impl ChargebackState for First {
    fn state_name() -> &'static str {
        "first"
    }
}

/// Representment stage - awaiting issuer decision.
///
/// At this stage, no actions are available. The merchant must wait
/// for the issuer to review the submitted evidence.
#[derive(Debug, Clone, Copy)]
pub struct Representment;

impl private::Sealed for Representment {}
impl ChargebackState for Representment {
    fn state_name() -> &'static str {
        "representment"
    }
}

/// Pre-arbitration stage - merchant must choose to arbitrate or accept.
///
/// Available actions:
/// - [`TypedChargeback::request_arbitration`] - Escalate to card network arbitration
/// - [`TypedChargeback::accept_liability`] - Accept the chargeback
/// - [`TypedChargeback::represent`] - Submit additional evidence (if allowed)
#[derive(Debug, Clone, Copy)]
pub struct PreArbitration;

impl private::Sealed for PreArbitration {}
impl ChargebackState for PreArbitration {
    fn state_name() -> &'static str {
        "preArbitration"
    }
}

/// Second chargeback stage - another round of dispute.
///
/// Available actions:
/// - [`TypedChargeback::represent`] - Submit evidence to dispute the chargeback
/// - [`TypedChargeback::accept_liability`] - Accept the chargeback
#[derive(Debug, Clone, Copy)]
pub struct SecondChargeback;

impl private::Sealed for SecondChargeback {}
impl ChargebackState for SecondChargeback {
    fn state_name() -> &'static str {
        "secondChargeback"
    }
}

/// Arbitration stage - awaiting card network decision.
///
/// At this stage, no actions are available. The merchant must wait
/// for the card network to make a final decision.
#[derive(Debug, Clone, Copy)]
pub struct Arbitration;

impl private::Sealed for Arbitration {}
impl ChargebackState for Arbitration {
    fn state_name() -> &'static str {
        "arbitration"
    }
}

/// Terminal stage - chargeback is closed.
///
/// The dispute has reached a final state (won, lost, or closed).
/// No further actions are available.
#[derive(Debug, Clone, Copy)]
pub struct Terminal;

impl private::Sealed for Terminal {}
impl ChargebackState for Terminal {
    fn state_name() -> &'static str {
        "terminal"
    }
}

// =============================================================================
// Section 2: TypedChargeback Wrapper
// =============================================================================

/// A chargeback with compile-time state enforcement.
///
/// This wrapper provides state-specific methods that are only available
/// when the chargeback is in the appropriate state. For example,
/// `represent()` is only available on `TypedChargeback<First>`.
///
/// # Type Parameters
///
/// * `S` - The current state of the chargeback (e.g., `First`, `PreArbitration`)
#[derive(Debug, Clone)]
pub struct TypedChargeback<S: ChargebackState> {
    inner: Chargeback,
    _state: PhantomData<S>,
}

impl<S: ChargebackState> TypedChargeback<S> {
    /// Create a new typed chargeback from raw chargeback data.
    fn new(chargeback: Chargeback) -> Self {
        Self {
            inner: chargeback,
            _state: PhantomData,
        }
    }

    /// Get a reference to the underlying chargeback data.
    pub fn inner(&self) -> &Chargeback {
        &self.inner
    }

    /// Consume the wrapper and return the underlying chargeback data.
    pub fn into_inner(self) -> Chargeback {
        self.inner
    }

    /// Get the chargeback ID.
    pub fn id(&self) -> &PayrixId {
        &self.inner.id
    }

    /// Get the current state name.
    pub fn state_name(&self) -> &'static str {
        S::state_name()
    }

    /// Get the chargeback amount in cents.
    pub fn amount(&self) -> Option<i64> {
        self.inner.total
    }

    /// Get the reason code for this chargeback.
    pub fn reason_code(&self) -> Option<&str> {
        self.inner.reason_code.as_deref()
    }

    /// Get the reason description for this chargeback.
    pub fn reason(&self) -> Option<&str> {
        self.inner.reason.as_deref()
    }

    /// Get the reply deadline as YYYYMMDD integer.
    pub fn reply_deadline(&self) -> Option<i32> {
        self.inner.reply
    }

    /// Check if this chargeback is actionable.
    pub fn is_actionable(&self) -> bool {
        self.inner.actionable
    }

    /// Get the associated merchant ID.
    pub fn merchant_id(&self) -> Option<&PayrixId> {
        self.inner.merchant.as_ref()
    }

    /// Get the associated transaction ID.
    pub fn transaction_id(&self) -> Option<&PayrixId> {
        self.inner.txn.as_ref()
    }
}

// =============================================================================
// Section 3: Evidence Types
// =============================================================================

/// Evidence document for chargeback representment.
///
/// Each document must be under 1 MB and in a supported format.
#[derive(Debug, Clone)]
pub struct EvidenceDocument {
    /// Document filename.
    pub name: String,
    /// Document content as bytes.
    pub content: Vec<u8>,
    /// MIME type (e.g., "application/pdf", "image/tiff").
    pub mime_type: String,
}

impl EvidenceDocument {
    /// Create a new evidence document.
    ///
    /// # Arguments
    ///
    /// * `name` - The filename (e.g., "receipt.pdf")
    /// * `content` - The file content as bytes
    /// * `mime_type` - The MIME type (e.g., "application/pdf")
    pub fn new(name: impl Into<String>, content: Vec<u8>, mime_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content,
            mime_type: mime_type.into(),
        }
    }

    /// Get the size of this document in bytes.
    pub fn size(&self) -> usize {
        self.content.len()
    }

    /// Validate this document against Payrix requirements.
    pub fn validate(&self) -> Result<()> {
        if self.content.len() > MAX_DOCUMENT_SIZE {
            return Err(Error::Validation(format!(
                "Document '{}' exceeds maximum size of {} bytes (actual: {} bytes)",
                self.name,
                MAX_DOCUMENT_SIZE,
                self.content.len()
            )));
        }

        // Validate MIME type
        let valid_types = [
            "image/tiff",
            "image/tif",
            "application/pdf",
            "image/png",
            "image/jpeg",
            "image/jpg",
            "image/gif",
        ];

        if !valid_types.contains(&self.mime_type.as_str()) {
            return Err(Error::Validation(format!(
                "Document '{}' has unsupported MIME type '{}'. Supported: {:?}",
                self.name, self.mime_type, valid_types
            )));
        }

        Ok(())
    }
}

/// Evidence for chargeback representment.
///
/// Contains a required message explaining the dispute and optional
/// supporting documents.
///
/// # Example
///
/// ```
/// use payrix::workflows::dispute_handling::Evidence;
///
/// let evidence = Evidence::new("Customer received the goods as described per tracking #1234")
///     .with_document("receipt.pdf", vec![/* pdf bytes */], "application/pdf")
///     .with_document("tracking.png", vec![/* image bytes */], "image/png");
/// ```
#[derive(Debug, Clone)]
pub struct Evidence {
    /// Required message explaining the dispute response.
    pub message: String,
    /// Supporting documents (max 8, max 8 MB total).
    pub documents: Vec<EvidenceDocument>,
}

impl Evidence {
    /// Create new evidence with a message.
    ///
    /// # Arguments
    ///
    /// * `message` - Explanation of why the chargeback should be reversed
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            documents: Vec::new(),
        }
    }

    /// Add a document to this evidence.
    ///
    /// # Arguments
    ///
    /// * `name` - The filename
    /// * `content` - The file content as bytes
    /// * `mime_type` - The MIME type
    pub fn with_document(
        mut self,
        name: impl Into<String>,
        content: Vec<u8>,
        mime_type: impl Into<String>,
    ) -> Self {
        self.documents
            .push(EvidenceDocument::new(name, content, mime_type));
        self
    }

    /// Add a pre-built evidence document.
    pub fn with_evidence_document(mut self, doc: EvidenceDocument) -> Self {
        self.documents.push(doc);
        self
    }

    /// Get the total size of all documents in bytes.
    pub fn total_size(&self) -> usize {
        self.documents.iter().map(|d| d.size()).sum()
    }

    /// Validate this evidence against Payrix requirements.
    ///
    /// Checks:
    /// - Message is not empty
    /// - Maximum 8 documents
    /// - Each document under 1 MB
    /// - Total size under 8 MB
    /// - All MIME types are supported
    pub fn validate(&self) -> Result<()> {
        if self.message.trim().is_empty() {
            return Err(Error::Validation(
                "Evidence message cannot be empty".to_string(),
            ));
        }

        if self.documents.len() > MAX_DOCUMENTS {
            return Err(Error::Validation(format!(
                "Too many documents: {} (maximum: {})",
                self.documents.len(),
                MAX_DOCUMENTS
            )));
        }

        let total_size = self.total_size();
        if total_size > MAX_TOTAL_SIZE {
            return Err(Error::Validation(format!(
                "Total document size {} bytes exceeds maximum of {} bytes",
                total_size, MAX_TOTAL_SIZE
            )));
        }

        for doc in &self.documents {
            doc.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Section 3b: Evidence Helper Functions
// =============================================================================

/// Create an evidence document from raw bytes.
///
/// Automatically infers MIME type from file extension.
///
/// # Arguments
///
/// * `filename` - The filename with extension (e.g., "receipt.pdf")
/// * `content` - The file content as bytes
pub fn evidence_from_bytes(filename: &str, content: Vec<u8>) -> Result<EvidenceDocument> {
    let mime_type = mime_type_from_extension(filename)?;
    let doc = EvidenceDocument::new(filename, content, mime_type);
    doc.validate()?;
    Ok(doc)
}

/// Create an evidence document from a file path.
///
/// Reads the file and automatically infers MIME type from extension.
///
/// # Arguments
///
/// * `path` - Path to the file
pub fn evidence_from_path(path: impl AsRef<Path>) -> Result<EvidenceDocument> {
    let path = path.as_ref();
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| Error::Validation("Invalid file path".to_string()))?;

    let content = std::fs::read(path).map_err(|e| Error::Io(e.to_string()))?;
    evidence_from_bytes(filename, content)
}

/// Create an evidence document from a base64 data URL.
///
/// Parses data URLs in the format: `data:[<mediatype>][;base64],<data>`
///
/// This is commonly used when receiving file uploads from browser JavaScript
/// using `FileReader.readAsDataURL()`.
///
/// # Arguments
///
/// * `filename` - The filename to use for this document
/// * `data_url` - The base64 data URL (e.g., "data:application/pdf;base64,JVBERi0...")
///
/// # Example
///
/// ```
/// use payrix::workflows::dispute_handling::evidence_from_base64_url;
///
/// // Parse a base64-encoded PDF from a browser upload
/// let doc = evidence_from_base64_url(
///     "receipt.pdf",
///     "data:application/pdf;base64,JVBERi0xLjQK"
/// );
/// ```
pub fn evidence_from_base64_url(filename: &str, data_url: &str) -> Result<EvidenceDocument> {
    // Parse data URL format: data:[<mediatype>][;base64],<data>
    let data_url = data_url
        .strip_prefix("data:")
        .ok_or_else(|| Error::Validation("Invalid data URL: must start with 'data:'".to_string()))?;

    let (header, data) = data_url.split_once(',').ok_or_else(|| {
        Error::Validation("Invalid data URL: missing comma separator".to_string())
    })?;

    // Parse header parts (e.g., "application/pdf;base64" or just "application/pdf")
    let parts: Vec<&str> = header.split(';').collect();
    let mime_type = parts.first().unwrap_or(&"application/octet-stream");

    // Check if base64 encoded
    let is_base64 = parts.iter().any(|p| *p == "base64");
    if !is_base64 {
        return Err(Error::Validation(
            "Only base64-encoded data URLs are supported".to_string(),
        ));
    }

    // Decode base64
    use base64::{engine::general_purpose::STANDARD, Engine};
    let content = STANDARD.decode(data).map_err(|e| {
        Error::Validation(format!("Invalid base64 in data URL: {}", e))
    })?;

    let doc = EvidenceDocument::new(filename, content, *mime_type);
    doc.validate()?;
    Ok(doc)
}

/// Infer MIME type from file extension.
fn mime_type_from_extension(filename: &str) -> Result<&'static str> {
    let ext = filename
        .rsplit('.')
        .next()
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "pdf" => Ok("application/pdf"),
        "tiff" | "tif" => Ok("image/tiff"),
        "png" => Ok("image/png"),
        "jpg" | "jpeg" => Ok("image/jpeg"),
        "gif" => Ok("image/gif"),
        _ => Err(Error::Validation(format!(
            "Unsupported file extension '{}'. Supported: pdf, tiff, tif, png, jpg, jpeg, gif",
            ext
        ))),
    }
}

// =============================================================================
// Section 4: State-Specific Methods
// =============================================================================

impl TypedChargeback<First> {
    /// Submit evidence to represent (dispute) this chargeback.
    ///
    /// This sends a response to the issuer with your evidence supporting
    /// that the transaction was valid.
    ///
    /// # Arguments
    ///
    /// * `client` - The Payrix client
    /// * `evidence` - Evidence supporting your case
    ///
    /// # Returns
    ///
    /// A `TypedChargeback<Representment>` representing the new state.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Evidence validation fails
    /// - The chargeback is not actionable
    /// - The API call fails
    pub async fn represent(
        self,
        client: &PayrixClient,
        evidence: Evidence,
    ) -> Result<TypedChargeback<Representment>> {
        evidence.validate()?;

        if !self.inner.actionable {
            return Err(Error::Validation(
                "Chargeback is not currently actionable".to_string(),
            ));
        }

        // Create the chargeback message
        let message = NewChargebackMessage {
            chargeback: self.inner.id.to_string(),
            message_type: Some(ChargebackMessageType::Represent),
            subject: Some("Representment".to_string()),
            message: Some(evidence.message),
        };

        let response: ChargebackMessage = client
            .create(EntityType::ChargebackMessages, &message)
            .await?;

        // Upload documents attached to this chargeback message
        for doc in evidence.documents {
            let document_type = mime_type_to_document_type(&doc.mime_type);
            let encoded_content = base64::engine::general_purpose::STANDARD.encode(&doc.content);

            let new_doc = NewChargebackDocument {
                chargeback: self.inner.id.to_string(),
                chargeback_message: Some(response.id.to_string()),
                name: Some(doc.name),
                document_type: Some(document_type),
                mime_type: Some(doc.mime_type),
                description: None,
                data: Some(encoded_content),
            };

            let _doc_response: ChargebackDocument = client
                .create(EntityType::ChargebackDocuments, &new_doc)
                .await?;
        }

        // Reload the chargeback to get the updated state
        let updated: Chargeback = client
            .get_one(EntityType::Chargebacks, self.inner.id.as_str())
            .await?
            .ok_or_else(|| Error::NotFound("Chargeback not found after update".to_string()))?;

        Ok(TypedChargeback::new(updated))
    }

    /// Accept liability for this chargeback.
    ///
    /// This acknowledges the chargeback and stops the dispute process.
    /// The chargeback amount will be deducted from your account.
    ///
    /// # Arguments
    ///
    /// * `client` - The Payrix client
    ///
    /// # Returns
    ///
    /// A `TypedChargeback<Terminal>` representing the closed state.
    pub async fn accept_liability(self, client: &PayrixClient) -> Result<TypedChargeback<Terminal>> {
        if !self.inner.actionable {
            return Err(Error::Validation(
                "Chargeback is not currently actionable".to_string(),
            ));
        }

        let message = NewChargebackMessage {
            chargeback: self.inner.id.to_string(),
            message_type: Some(ChargebackMessageType::AcceptLiability),
            subject: Some("Accept Liability".to_string()),
            message: Some("Merchant accepts liability for this chargeback".to_string()),
        };

        let _response: ChargebackMessage = client
            .create(EntityType::ChargebackMessages, &message)
            .await?;

        let updated: Chargeback = client
            .get_one(EntityType::Chargebacks, self.inner.id.as_str())
            .await?
            .ok_or_else(|| Error::NotFound("Chargeback not found after update".to_string()))?;

        Ok(TypedChargeback::new(updated))
    }
}

impl TypedChargeback<PreArbitration> {
    /// Request arbitration from the card network.
    ///
    /// This escalates the dispute to the card network (Visa, Mastercard, etc.)
    /// for a final binding decision. There is typically a fee for arbitration,
    /// which may be refunded if you win.
    ///
    /// # Arguments
    ///
    /// * `client` - The Payrix client
    ///
    /// # Returns
    ///
    /// A `TypedChargeback<Arbitration>` representing the arbitration stage.
    pub async fn request_arbitration(
        self,
        client: &PayrixClient,
    ) -> Result<TypedChargeback<Arbitration>> {
        if !self.inner.actionable {
            return Err(Error::Validation(
                "Chargeback is not currently actionable".to_string(),
            ));
        }

        let message = NewChargebackMessage {
            chargeback: self.inner.id.to_string(),
            message_type: Some(ChargebackMessageType::RequestArbitration),
            subject: Some("Request Arbitration".to_string()),
            message: Some("Merchant requests card network arbitration".to_string()),
        };

        let _response: ChargebackMessage = client
            .create(EntityType::ChargebackMessages, &message)
            .await?;

        let updated: Chargeback = client
            .get_one(EntityType::Chargebacks, self.inner.id.as_str())
            .await?
            .ok_or_else(|| Error::NotFound("Chargeback not found after update".to_string()))?;

        Ok(TypedChargeback::new(updated))
    }

    /// Submit additional evidence in pre-arbitration.
    ///
    /// Some card networks allow submitting additional evidence during
    /// pre-arbitration before escalating to full arbitration.
    pub async fn represent(
        self,
        client: &PayrixClient,
        evidence: Evidence,
    ) -> Result<TypedChargeback<Representment>> {
        evidence.validate()?;

        if !self.inner.actionable {
            return Err(Error::Validation(
                "Chargeback is not currently actionable".to_string(),
            ));
        }

        let message = NewChargebackMessage {
            chargeback: self.inner.id.to_string(),
            message_type: Some(ChargebackMessageType::Represent),
            subject: Some("Pre-Arbitration Response".to_string()),
            message: Some(evidence.message),
        };

        let response: ChargebackMessage = client
            .create(EntityType::ChargebackMessages, &message)
            .await?;

        // Upload documents attached to this chargeback message
        for doc in evidence.documents {
            let document_type = mime_type_to_document_type(&doc.mime_type);
            let encoded_content = base64::engine::general_purpose::STANDARD.encode(&doc.content);

            let new_doc = NewChargebackDocument {
                chargeback: self.inner.id.to_string(),
                chargeback_message: Some(response.id.to_string()),
                name: Some(doc.name),
                document_type: Some(document_type),
                mime_type: Some(doc.mime_type),
                description: None,
                data: Some(encoded_content),
            };

            let _doc_response: ChargebackDocument = client
                .create(EntityType::ChargebackDocuments, &new_doc)
                .await?;
        }

        let updated: Chargeback = client
            .get_one(EntityType::Chargebacks, self.inner.id.as_str())
            .await?
            .ok_or_else(|| Error::NotFound("Chargeback not found after update".to_string()))?;

        Ok(TypedChargeback::new(updated))
    }

    /// Accept liability for this chargeback.
    pub async fn accept_liability(self, client: &PayrixClient) -> Result<TypedChargeback<Terminal>> {
        if !self.inner.actionable {
            return Err(Error::Validation(
                "Chargeback is not currently actionable".to_string(),
            ));
        }

        let message = NewChargebackMessage {
            chargeback: self.inner.id.to_string(),
            message_type: Some(ChargebackMessageType::AcceptLiability),
            subject: Some("Accept Liability".to_string()),
            message: Some("Merchant accepts liability for this chargeback".to_string()),
        };

        let _response: ChargebackMessage = client
            .create(EntityType::ChargebackMessages, &message)
            .await?;

        let updated: Chargeback = client
            .get_one(EntityType::Chargebacks, self.inner.id.as_str())
            .await?
            .ok_or_else(|| Error::NotFound("Chargeback not found after update".to_string()))?;

        Ok(TypedChargeback::new(updated))
    }
}

impl TypedChargeback<SecondChargeback> {
    /// Submit evidence to represent this second chargeback.
    pub async fn represent(
        self,
        client: &PayrixClient,
        evidence: Evidence,
    ) -> Result<TypedChargeback<Representment>> {
        evidence.validate()?;

        if !self.inner.actionable {
            return Err(Error::Validation(
                "Chargeback is not currently actionable".to_string(),
            ));
        }

        let message = NewChargebackMessage {
            chargeback: self.inner.id.to_string(),
            message_type: Some(ChargebackMessageType::Represent),
            subject: Some("Second Chargeback Representment".to_string()),
            message: Some(evidence.message),
        };

        let response: ChargebackMessage = client
            .create(EntityType::ChargebackMessages, &message)
            .await?;

        // Upload documents attached to this chargeback message
        for doc in evidence.documents {
            let document_type = mime_type_to_document_type(&doc.mime_type);
            let encoded_content = base64::engine::general_purpose::STANDARD.encode(&doc.content);

            let new_doc = NewChargebackDocument {
                chargeback: self.inner.id.to_string(),
                chargeback_message: Some(response.id.to_string()),
                name: Some(doc.name),
                document_type: Some(document_type),
                mime_type: Some(doc.mime_type),
                description: None,
                data: Some(encoded_content),
            };

            let _doc_response: ChargebackDocument = client
                .create(EntityType::ChargebackDocuments, &new_doc)
                .await?;
        }

        let updated: Chargeback = client
            .get_one(EntityType::Chargebacks, self.inner.id.as_str())
            .await?
            .ok_or_else(|| Error::NotFound("Chargeback not found after update".to_string()))?;

        Ok(TypedChargeback::new(updated))
    }

    /// Accept liability for this chargeback.
    pub async fn accept_liability(self, client: &PayrixClient) -> Result<TypedChargeback<Terminal>> {
        if !self.inner.actionable {
            return Err(Error::Validation(
                "Chargeback is not currently actionable".to_string(),
            ));
        }

        let message = NewChargebackMessage {
            chargeback: self.inner.id.to_string(),
            message_type: Some(ChargebackMessageType::AcceptLiability),
            subject: Some("Accept Liability".to_string()),
            message: Some("Merchant accepts liability for this chargeback".to_string()),
        };

        let _response: ChargebackMessage = client
            .create(EntityType::ChargebackMessages, &message)
            .await?;

        let updated: Chargeback = client
            .get_one(EntityType::Chargebacks, self.inner.id.as_str())
            .await?
            .ok_or_else(|| Error::NotFound("Chargeback not found after update".to_string()))?;

        Ok(TypedChargeback::new(updated))
    }
}

// =============================================================================
// Section 5: Runtime Bridge
// =============================================================================

/// An active (non-terminal) chargeback dispute.
///
/// This enum dispatches to the appropriate typed state based on the
/// chargeback's current cycle.
#[derive(Debug, Clone)]
pub enum ActiveDispute {
    /// Retrieval stage - awaiting first chargeback.
    Retrieval(TypedChargeback<Retrieval>),
    /// First chargeback stage - can represent or accept.
    First(TypedChargeback<First>),
    /// Representment stage - awaiting issuer decision.
    Representment(TypedChargeback<Representment>),
    /// Pre-arbitration stage - can arbitrate, represent, or accept.
    PreArbitration(TypedChargeback<PreArbitration>),
    /// Second chargeback stage - can represent or accept.
    SecondChargeback(TypedChargeback<SecondChargeback>),
    /// Arbitration stage - awaiting card network decision.
    Arbitration(TypedChargeback<Arbitration>),
}

impl ActiveDispute {
    /// Get the underlying chargeback ID.
    pub fn id(&self) -> &PayrixId {
        match self {
            Self::Retrieval(c) => c.id(),
            Self::First(c) => c.id(),
            Self::Representment(c) => c.id(),
            Self::PreArbitration(c) => c.id(),
            Self::SecondChargeback(c) => c.id(),
            Self::Arbitration(c) => c.id(),
        }
    }

    /// Get the state name.
    pub fn state_name(&self) -> &'static str {
        match self {
            Self::Retrieval(_) => Retrieval::state_name(),
            Self::First(_) => First::state_name(),
            Self::Representment(_) => Representment::state_name(),
            Self::PreArbitration(_) => PreArbitration::state_name(),
            Self::SecondChargeback(_) => SecondChargeback::state_name(),
            Self::Arbitration(_) => Arbitration::state_name(),
        }
    }

    /// Get a reference to the underlying chargeback data.
    pub fn inner(&self) -> &Chargeback {
        match self {
            Self::Retrieval(c) => c.inner(),
            Self::First(c) => c.inner(),
            Self::Representment(c) => c.inner(),
            Self::PreArbitration(c) => c.inner(),
            Self::SecondChargeback(c) => c.inner(),
            Self::Arbitration(c) => c.inner(),
        }
    }
}

/// A chargeback dispute - either active or terminal.
///
/// This is the primary entry point for working with chargebacks.
/// Use [`ChargebackDispute::load`] to fetch a chargeback from the API.
#[derive(Debug, Clone)]
pub enum ChargebackDispute {
    /// An active dispute that may have available actions.
    Active(ActiveDispute),
    /// A terminal dispute that is closed.
    Terminal(TypedChargeback<Terminal>),
}

impl ChargebackDispute {
    /// Load a chargeback from the Payrix API.
    ///
    /// This is the primary entry point for working with chargebacks.
    /// The returned dispute is typed according to the chargeback's current state.
    ///
    /// # Arguments
    ///
    /// * `client` - The Payrix client
    /// * `id` - The chargeback ID
    ///
    /// # Returns
    ///
    /// A `ChargebackDispute` in the appropriate state.
    pub async fn load(client: &PayrixClient, id: &str) -> Result<Self> {
        let chargeback: Chargeback = client
            .get_one(EntityType::Chargebacks, id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Chargeback not found: {}", id)))?;

        Ok(Self::from_chargeback(chargeback))
    }

    /// Convert a raw chargeback into a typed dispute.
    ///
    /// Useful when you have chargeback data from a webhook or other source.
    pub fn from_chargeback(chargeback: Chargeback) -> Self {
        // Check if terminal first
        if let Some(status) = &chargeback.status {
            match status {
                ChargebackStatusValue::Closed
                | ChargebackStatusValue::Won
                | ChargebackStatusValue::Lost => {
                    return Self::Terminal(TypedChargeback::new(chargeback));
                }
                _ => {}
            }
        }

        // Check cycle for terminal states
        if let Some(cycle) = &chargeback.cycle {
            match cycle {
                ChargebackCycle::ArbitrationWon
                | ChargebackCycle::ArbitrationLost
                | ChargebackCycle::ArbitrationSplit
                | ChargebackCycle::Reversal => {
                    return Self::Terminal(TypedChargeback::new(chargeback));
                }
                _ => {}
            }
        }

        // Map to active state based on cycle
        let active = match chargeback.cycle {
            Some(ChargebackCycle::Retrieval) => {
                ActiveDispute::Retrieval(TypedChargeback::new(chargeback))
            }
            Some(ChargebackCycle::First) => ActiveDispute::First(TypedChargeback::new(chargeback)),
            Some(ChargebackCycle::Representment) => {
                ActiveDispute::Representment(TypedChargeback::new(chargeback))
            }
            Some(ChargebackCycle::PreArbitration)
            | Some(ChargebackCycle::IssuerDeclinedPreArbitration)
            | Some(ChargebackCycle::ResponseToIssuerPreArbitration)
            | Some(ChargebackCycle::MerchantDeclinedPreArbitration) => {
                ActiveDispute::PreArbitration(TypedChargeback::new(chargeback))
            }
            Some(ChargebackCycle::Arbitration)
            | Some(ChargebackCycle::PreCompliance)
            | Some(ChargebackCycle::Compliance) => {
                ActiveDispute::Arbitration(TypedChargeback::new(chargeback))
            }
            // Default to First for unknown or None
            None => ActiveDispute::First(TypedChargeback::new(chargeback)),
            _ => ActiveDispute::First(TypedChargeback::new(chargeback)),
        };

        Self::Active(active)
    }

    /// Refresh this dispute with the latest data from the API.
    ///
    /// Returns a new `ChargebackDispute` with the updated state.
    pub async fn refresh(&self, client: &PayrixClient) -> Result<Self> {
        Self::load(client, self.id().as_str()).await
    }

    /// Get the chargeback ID.
    pub fn id(&self) -> &PayrixId {
        match self {
            Self::Active(active) => active.id(),
            Self::Terminal(terminal) => terminal.id(),
        }
    }

    /// Get the state name.
    pub fn state_name(&self) -> &'static str {
        match self {
            Self::Active(active) => active.state_name(),
            Self::Terminal(_) => Terminal::state_name(),
        }
    }

    /// Get a reference to the underlying chargeback data.
    pub fn inner(&self) -> &Chargeback {
        match self {
            Self::Active(active) => active.inner(),
            Self::Terminal(terminal) => terminal.inner(),
        }
    }

    /// Check if this dispute is terminal (closed).
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Terminal(_))
    }

    /// Check if this dispute is active (not closed).
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }
}

// =============================================================================
// Section 6: Convenience Functions
// =============================================================================

/// Get all actionable chargebacks for a merchant.
///
/// Returns chargebacks that are open and have available actions.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `merchant_id` - The merchant ID to filter by
pub async fn get_actionable_disputes(
    client: &PayrixClient,
    merchant_id: &str,
) -> Result<Vec<ChargebackDispute>> {
    let search = format!(
        "merchant[equals]={}&status[equals]=open&actionable[equals]=1",
        merchant_id
    );

    let chargebacks: Vec<Chargeback> = client.search(EntityType::Chargebacks, &search).await?;

    Ok(chargebacks
        .into_iter()
        .map(ChargebackDispute::from_chargeback)
        .collect())
}

/// Get chargebacks for a merchant filtered by cycle.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `merchant_id` - The merchant ID to filter by
/// * `cycle` - The chargeback cycle to filter by
pub async fn get_disputes_by_cycle(
    client: &PayrixClient,
    merchant_id: &str,
    cycle: ChargebackCycle,
) -> Result<Vec<ChargebackDispute>> {
    let cycle_str = match cycle {
        ChargebackCycle::Retrieval => "retrieval",
        ChargebackCycle::First => "first",
        ChargebackCycle::Representment => "representment",
        ChargebackCycle::PreArbitration => "preArbitration",
        ChargebackCycle::Arbitration => "arbitration",
        _ => return Ok(Vec::new()), // Terminal states don't need searching
    };

    let search = format!(
        "merchant[equals]={}&cycle[equals]={}",
        merchant_id, cycle_str
    );

    let chargebacks: Vec<Chargeback> = client.search(EntityType::Chargebacks, &search).await?;

    Ok(chargebacks
        .into_iter()
        .map(ChargebackDispute::from_chargeback)
        .collect())
}

/// Get all chargebacks for a specific transaction.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `transaction_id` - The transaction ID
pub async fn get_disputes_for_transaction(
    client: &PayrixClient,
    transaction_id: &str,
) -> Result<Vec<ChargebackDispute>> {
    let search = format!("txn[equals]={}", transaction_id);

    let chargebacks: Vec<Chargeback> = client.search(EntityType::Chargebacks, &search).await?;

    Ok(chargebacks
        .into_iter()
        .map(ChargebackDispute::from_chargeback)
        .collect())
}

// =============================================================================
// Section 6b: Helper Functions
// =============================================================================

/// Convert a MIME type to a ChargebackDocumentType.
fn mime_type_to_document_type(mime_type: &str) -> ChargebackDocumentType {
    match mime_type.to_lowercase().as_str() {
        "application/pdf" => ChargebackDocumentType::Pdf,
        "image/tiff" | "image/tif" => ChargebackDocumentType::Tiff,
        "image/png" => ChargebackDocumentType::Png,
        "image/jpeg" | "image/jpg" => ChargebackDocumentType::Jpg,
        "image/gif" => ChargebackDocumentType::Image,
        "text/plain" => ChargebackDocumentType::Text,
        _ if mime_type.starts_with("image/") => ChargebackDocumentType::Image,
        _ => ChargebackDocumentType::Other,
    }
}

// =============================================================================
// Section 7: Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // State Marker Tests
    // =========================================================================

    #[test]
    fn test_state_names() {
        assert_eq!(Retrieval::state_name(), "retrieval");
        assert_eq!(First::state_name(), "first");
        assert_eq!(Representment::state_name(), "representment");
        assert_eq!(PreArbitration::state_name(), "preArbitration");
        assert_eq!(SecondChargeback::state_name(), "secondChargeback");
        assert_eq!(Arbitration::state_name(), "arbitration");
        assert_eq!(Terminal::state_name(), "terminal");
    }

    // =========================================================================
    // Evidence Validation Tests
    // =========================================================================

    #[test]
    fn test_evidence_creation() {
        let evidence = Evidence::new("Test message");
        assert_eq!(evidence.message, "Test message");
        assert!(evidence.documents.is_empty());
    }

    #[test]
    fn test_evidence_with_document() {
        let evidence = Evidence::new("Test message")
            .with_document("receipt.pdf", vec![1, 2, 3], "application/pdf");

        assert_eq!(evidence.documents.len(), 1);
        assert_eq!(evidence.documents[0].name, "receipt.pdf");
        assert_eq!(evidence.documents[0].mime_type, "application/pdf");
    }

    #[test]
    fn test_evidence_validation_empty_message() {
        let evidence = Evidence::new("");
        let result = evidence.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_evidence_validation_whitespace_message() {
        let evidence = Evidence::new("   ");
        let result = evidence.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_evidence_validation_too_many_documents() {
        let mut evidence = Evidence::new("Test");
        for i in 0..9 {
            evidence = evidence.with_document(
                format!("doc{}.pdf", i),
                vec![1],
                "application/pdf",
            );
        }

        let result = evidence.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Too many documents"));
    }

    #[test]
    fn test_evidence_validation_document_too_large() {
        let large_content = vec![0u8; MAX_DOCUMENT_SIZE + 1];
        let evidence = Evidence::new("Test").with_document("large.pdf", large_content, "application/pdf");

        let result = evidence.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds maximum size"));
    }

    #[test]
    fn test_evidence_validation_invalid_mime_type() {
        let evidence = Evidence::new("Test").with_document("file.exe", vec![1], "application/exe");

        let result = evidence.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unsupported MIME type"));
    }

    #[test]
    fn test_evidence_validation_valid() {
        let evidence = Evidence::new("Valid evidence message")
            .with_document("receipt.pdf", vec![1, 2, 3], "application/pdf")
            .with_document("photo.png", vec![4, 5, 6], "image/png");

        assert!(evidence.validate().is_ok());
    }

    #[test]
    fn test_evidence_total_size() {
        let evidence = Evidence::new("Test")
            .with_document("a.pdf", vec![1, 2, 3], "application/pdf")
            .with_document("b.pdf", vec![4, 5, 6, 7], "application/pdf");

        assert_eq!(evidence.total_size(), 7);
    }

    // =========================================================================
    // Evidence Helper Tests
    // =========================================================================

    #[test]
    fn test_mime_type_from_extension() {
        assert_eq!(mime_type_from_extension("file.pdf").unwrap(), "application/pdf");
        assert_eq!(mime_type_from_extension("file.PDF").unwrap(), "application/pdf");
        assert_eq!(mime_type_from_extension("file.tiff").unwrap(), "image/tiff");
        assert_eq!(mime_type_from_extension("file.tif").unwrap(), "image/tiff");
        assert_eq!(mime_type_from_extension("file.png").unwrap(), "image/png");
        assert_eq!(mime_type_from_extension("file.jpg").unwrap(), "image/jpeg");
        assert_eq!(mime_type_from_extension("file.jpeg").unwrap(), "image/jpeg");
        assert_eq!(mime_type_from_extension("file.gif").unwrap(), "image/gif");
    }

    #[test]
    fn test_mime_type_unsupported() {
        assert!(mime_type_from_extension("file.exe").is_err());
        assert!(mime_type_from_extension("file.doc").is_err());
    }

    #[test]
    fn test_evidence_from_bytes() {
        let doc = evidence_from_bytes("receipt.pdf", vec![1, 2, 3]).unwrap();
        assert_eq!(doc.name, "receipt.pdf");
        assert_eq!(doc.mime_type, "application/pdf");
    }

    #[test]
    fn test_evidence_from_base64_url() {
        // "test" in base64 is "dGVzdA=="
        let doc = evidence_from_base64_url("file.pdf", "data:application/pdf;base64,dGVzdA==").unwrap();
        assert_eq!(doc.name, "file.pdf");
        assert_eq!(doc.mime_type, "application/pdf");
        assert_eq!(doc.content, b"test");
    }

    #[test]
    fn test_evidence_from_base64_url_invalid() {
        // Missing data: prefix
        assert!(evidence_from_base64_url("file.pdf", "application/pdf;base64,dGVzdA==").is_err());

        // Missing comma
        assert!(evidence_from_base64_url("file.pdf", "data:application/pdf;base64").is_err());

        // Not base64 encoded
        assert!(evidence_from_base64_url("file.pdf", "data:application/pdf,notbase64").is_err());
    }

    // =========================================================================
    // ChargebackDispute Tests
    // =========================================================================

    fn make_test_chargeback(cycle: Option<ChargebackCycle>, status: Option<ChargebackStatusValue>) -> Chargeback {
        Chargeback {
            // t1_chb_12345678901234567890123 is exactly 30 characters
            id: "t1_chb_12345678901234567890123".parse().unwrap(),
            created: None,
            modified: None,
            creator: None,
            modifier: None,
            merchant: None,
            txn: None,
            mid: None,
            description: None,
            total: Some(10000),
            represented_total: None,
            cycle,
            currency: Some("USD".to_string()),
            platform: None,
            payment_method: None,
            reference: None,
            reason: Some("Disputed charge".to_string()),
            reason_code: Some("4853".to_string()),
            issued: None,
            received: None,
            reply: Some(20240130),
            bank_ref: None,
            chargeback_ref: None,
            status,
            last_status_change: None,
            actionable: true,
            shadow: false,
            inactive: false,
            frozen: false,
        }
    }

    #[test]
    fn test_from_chargeback_first() {
        let cb = make_test_chargeback(Some(ChargebackCycle::First), Some(ChargebackStatusValue::Open));
        let dispute = ChargebackDispute::from_chargeback(cb);

        assert!(dispute.is_active());
        assert!(!dispute.is_terminal());
        assert_eq!(dispute.state_name(), "first");

        if let ChargebackDispute::Active(ActiveDispute::First(_)) = dispute {
            // Good - it's in First state
        } else {
            panic!("Expected First state");
        }
    }

    #[test]
    fn test_from_chargeback_pre_arbitration() {
        let cb = make_test_chargeback(Some(ChargebackCycle::PreArbitration), Some(ChargebackStatusValue::Open));
        let dispute = ChargebackDispute::from_chargeback(cb);

        assert!(dispute.is_active());
        assert_eq!(dispute.state_name(), "preArbitration");

        if let ChargebackDispute::Active(ActiveDispute::PreArbitration(_)) = dispute {
            // Good - it's in PreArbitration state
        } else {
            panic!("Expected PreArbitration state");
        }
    }

    #[test]
    fn test_from_chargeback_terminal_won() {
        let cb = make_test_chargeback(Some(ChargebackCycle::ArbitrationWon), Some(ChargebackStatusValue::Won));
        let dispute = ChargebackDispute::from_chargeback(cb);

        assert!(dispute.is_terminal());
        assert!(!dispute.is_active());
        assert_eq!(dispute.state_name(), "terminal");
    }

    #[test]
    fn test_from_chargeback_terminal_closed() {
        let cb = make_test_chargeback(Some(ChargebackCycle::First), Some(ChargebackStatusValue::Closed));
        let dispute = ChargebackDispute::from_chargeback(cb);

        assert!(dispute.is_terminal());
    }

    #[test]
    fn test_typed_chargeback_accessors() {
        let cb = make_test_chargeback(Some(ChargebackCycle::First), Some(ChargebackStatusValue::Open));
        let dispute = ChargebackDispute::from_chargeback(cb);

        assert_eq!(dispute.inner().total, Some(10000));
        assert_eq!(dispute.inner().reason_code.as_deref(), Some("4853"));
        assert_eq!(dispute.inner().reply, Some(20240130));
    }

    #[test]
    fn test_active_dispute_id() {
        let cb = make_test_chargeback(Some(ChargebackCycle::First), Some(ChargebackStatusValue::Open));
        let id = cb.id.clone();
        let dispute = ChargebackDispute::from_chargeback(cb);

        assert_eq!(dispute.id().as_str(), id.as_str());
    }
}
