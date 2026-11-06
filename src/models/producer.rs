// Producer model per data-model.md Traceability Domain entity
// Fields: id, user_id UNIQUE reference to platform.users, business_name, registration_number,
// description, verification_status, created_at, updated_at

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use time::OffsetDateTime;

/// Producer platform entity
/// Owned by: Traceability Domain (constitution.md §164-178)
/// Responsible for: Product provenance and traceability integrity (constitution.md §178)
/// Relationships:
///   - ONE-TO-ONE with User (user_id → platform.users.id) (application-level contract)
///   - ONE-TO-MANY with Farm (producer_id → traceability.farms.producer_id)
///   - ONE-TO-MANY with Certification (producer_id → traceability.certifications.producer_id)
///   - ONE-TO-MANY with Batch (producer_id → traceability.batches.producer_id)

/// Producer verification status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Suspended,
    Revoked,
}

/// Producer platform entity
/// Fields: id, user_id UNIQUE, business_name, registration_number, description,
/// verification_status, created_at, updated_at
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Producer {
    /// Unique identifier
    pub id: Uuid,
    /// User ID reference (platform domain, UNIQUE per contract)
    pub user_id: uuid::Uuid,
    /// Business name
    pub business_name: String,
    /// Registration number (optional)
    pub registration_number: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Verification status
    pub verification_status: VerificationStatus,
    /// When the producer was created
    pub created_at: OffsetDateTime,
    /// When the producer was last updated
    pub updated_at: OffsetDateTime,
}

/// Producer verification status checks
impl Producer {
    /// Check if producer is verified
    pub fn is_verified(&self) -> bool {
        matches!(self.verification_status, VerificationStatus::Verified)
    }

    /// Check if producer is pending verification
    pub fn is_pending(&self) -> bool {
        matches!(self.verification_status, VerificationStatus::Pending)
    }

    /// Check if producer is suspended or revoked
    pub fn is_suspended_or_revoked(&self) -> bool {
        matches!(
            self.verification_status,
            VerificationStatus::Suspended | VerificationStatus::Revoked
        )
    }

    /// Check if producer can certify products
    pub fn can_certify(&self) -> bool {
        matches!(self.verification_status, VerificationStatus::Verified)
    }
}

/// Producer creation helper
impl Producer {
    /// Create a new producer with Pending verification
    pub fn new(user_id: uuid::Uuid, business_name: String) -> Self {
        let now = OffsetDateTime::now_local()
            .expect("Failed to get current time");
        Producer {
            id: uuid::Uuid::new_v4(),
            user_id,
            business_name,
            registration_number: None,
            description: None,
            verification_status: VerificationStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set registration number
    pub fn with_registration_number(mut self, reg_number: String) -> Self {
        self.registration_number = Some(reg_number);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set verification status
    pub fn with_verification_status(mut self, status: VerificationStatus) -> Self {
        self.verification_status = status;
        self
    }

    /// Convert Producer to API response DTO (no sensitive user data)
    pub fn to_dto(&self) -> ProducerDto {
        ProducerDto {
            id: self.id,
            business_name: self.business_name.clone(),
            registration_number: self.registration_number.clone(),
            verification_status: self.verification_status,
        }
    }
}

/// Producer DTO for API responses
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ProducerDto {
        /// Unique identifier
        pub id: Uuid,
        /// Business name
        pub business_name: String,
        /// Registration number (optional)
        pub registration_number: Option<String>,
        /// Verification status
        pub verification_status: VerificationStatus,
    }
}