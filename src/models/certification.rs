// Certification model per data-model.md Traceability Domain entity
// Fields: id, producer_id OR farm_id (one required), certification_type, certifying_body,
// certificate_number, valid_from/valid_to, verification_status, verified_at, created_at, updated_at

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use time::OffsetDateTime;

/// Certification verification status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationStatus {
    Pending,
    Valid,
    Expired,
    Rejected,
}

/// Certification platform entity
/// Owned by: Traceability Domain (constitution.md §164-178)
/// Responsible for: Product provenance and traceability integrity (constitution.md §178)
/// Relationships:
///   - MANY-TO-ONE to Producer (producer_id, optional) — controlled association
///   - MANY-TO-ONE to Farm (farm_id, optional) — controlled association
///   - Constraint: at least producer_id OR farm_id must be set (ck_certification_subject)
///   - Certification scope must be explicit (database design §962)

/// Certification verification status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationStatus {
    Pending,
    Valid,
    Expired,
    Rejected,
}

/// Certification platform entity
/// Fields: id, producer_id OR farm_id (one required), certification_type, certifying_body,
/// certificate_number, valid_from/valid_to, verification_status, verified_at, created_at, updated_at
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Certification {
    /// Unique identifier
    pub id: Uuid,
    /// Producer reference (optional - controlled association)
    pub producer_id: Option<uuid::Uuid>,
    /// Farm reference (optional - controlled association)
    pub farm_id: Option<uuid::Uuid>,
    /// Certification type (e.g., "Organic", "Bio", "Rainforest Alliance")
    pub certification_type: String,
    /// Certifying body
    pub certifying_body: String,
    /// Certificate number
    pub certificate_number: String,
    /// When certification becomes valid
    pub valid_from: OffsetDateTime,
    /// When certification expires
    pub valid_to: OffsetDateTime,
    /// Verification status
    pub verification_status: CertificationStatus,
    /// When certification was verified (if applicable)
    pub verified_at: Option<OffsetDateTime>,
    /// When the certification was created
    pub created_at: OffsetDateTime,
    /// When the certification was last updated
    pub updated_at: OffsetDateTime,
}

/// Certification status checks
impl Certification {
    /// Check if certification is valid (current date within valid_from..valid_to)
    pub fn is_valid(&self, current_date: OffsetDateTime) -> bool {
        current_date >= self.valid_from && current_date <= self.valid_to
    }

    /// Check if certification is expired
    pub fn is_expired(&self, current_date: OffsetDateTime) -> bool {
        current_date > self.valid_to
    }

    /// Check if certification is pending (future-effective)
    pub fn is_pending(&self, current_date: OffsetDateTime) -> bool {
        current_date < self.valid_from
    }

    /// Check if certification can be used for product traceability
    pub fn is_usable(&self, current_date: OffsetDateTime) -> bool {
        !self.is_expired(current_date) && !matches!(self.verification_status, CertificationStatus::Rejected)
    }

    /// Check if at least one certification source is provided (producer_id OR farm_id)
    pub fn has_source(&self) -> bool {
        self.producer_id.is_some() || self.farm_id.is_some()
    }
}

/// Certification creation helper
impl Certification {
    /// Create a new certification with Pending status
    pub fn new(
        certification_type: String,
        certifying_body: String,
        certificate_number: String,
        valid_from: OffsetDateTime,
        valid_to: OffsetDateTime,
    ) -> Self {
        let now = OffsetDateTime::now_local()
            .expect("Failed to get current time");
        Certification {
            id: uuid::Uuid::new_v4(),
            producer_id: None,
            farm_id: None,
            certification_type,
            certifying_body,
            certificate_number,
            valid_from,
            valid_to,
            verification_status: CertificationStatus::Pending,
            verified_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set producer association (controlled)
    pub fn with_producer(mut self, producer_id: uuid::Uuid) -> Self {
        self.producer_id = Some(producer_id);
        self
    }

    /// Set farm association (controlled)
    pub fn with_farm(mut self, farm_id: uuid::Uuid) -> Self {
        self.farm_id = Some(farm_id);
        self
    }

    /// Set verification status
    pub fn with_verification_status(mut self, status: CertificationStatus) -> Self {
        self.verification_status = status;
        self
    }

    /// Convert Certification to API response DTO (no external references)
    pub fn to_dto(&self) -> CertificationDto {
        CertificationDto {
            id: self.id,
            certification_type: self.certification_type.clone(),
            certifying_body: self.certifying_body.clone(),
            certificate_number: self.certificate_number.clone(),
            valid_from: self.valid_from,
            valid_to: self.valid_to,
            verification_status: self.verification_status,
            has_source: self.has_source(),
        }
    }
}

/// Certification DTO for API responses
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CertificationDto {
        /// Unique identifier
        pub id: Uuid,
        /// Certification type
        pub certification_type: String,
        /// Certifying body
        pub certifying_body: String,
        /// Certificate number
        pub certificate_number: String,
        /// When certification becomes valid
        pub valid_from: OffsetDateTime,
        /// When certification expires
        pub valid_to: OffsetDateTime,
        /// Verification status
        pub verification_status: CertificationStatus,
        /// Whether at least one source (producer/farm) is provided
        pub has_source: bool,
    }
}