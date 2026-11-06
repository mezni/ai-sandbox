// Farm model per data-model.md Traceability Domain entity
// Fields: id, producer_id NOT NULL reference to traceability.producers, name,
// location_description, farm_type, status, created_at, updated_at

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use time::OffsetDateTime;

/// Farm status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FarmStatus {
    Active,
    Inactive,
    Suspended,
}

/// Farm platform entity
/// Owned by: Traceability Domain (constitution.md §164-178)
/// Responsible for: Producer farm management, location tracking (constitution.md §164-178)
/// Relationships:
///   - MANY-TO-ONE to Producer (producer_id → traceability.producers.id)
///   - MANY-TO-ONE with Certification (farm_id → traceability.certifications.farm_id) (optional)
///   - MANY-TO-ONE with Batch (farm_id → traceability.batches.farm_id)

/// Farm platform entity
/// Fields: id, producer_id NOT NULL, name, location_description, farm_type, status, created_at, updated_at
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Farm {
    /// Unique identifier
    pub id: Uuid,
    /// Producer ID (NOT NULL - farm belongs to exactly one producer)
    pub producer_id: uuid::Uuid,
    /// Farm name
    pub name: String,
    /// Location description
    pub location_description: String,
    /// Farm type
    pub farm_type: String,
    /// Farm status
    pub status: FarmStatus,
    /// When the farm was created
    pub created_at: OffsetDateTime,
    /// When the farm was last updated
    pub updated_at: OffsetDateTime,
}

/// Farm status checks
impl Farm {
    /// Check if farm is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, FarmStatus::Active)
    }

    /// Check if farm is inactive
    pub fn is_inactive(&self) -> bool {
        matches!(self.status, FarmStatus::Inactive)
    }

    /// Check if farm is suspended
    pub fn is_suspended(&self) -> bool {
        matches!(self.status, FarmStatus::Suspended)
    }

    /// Check if farm can have certifications
    pub fn can_have_certifications(&self) -> bool {
        self.status == FarmStatus::Active
    }
}

/// Farm creation helper
impl Farm {
    /// Create a new farm with Active status
    pub fn new(producer_id: uuid::Uuid, name: String, location_description: String, farm_type: String) -> Self {
        let now = OffsetDateTime::now_local()
            .expect("Failed to get current time");
        Farm {
            id: uuid::Uuid::new_v4(),
            producer_id,
            name,
            location_description,
            farm_type,
            status: FarmStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set farm status
    pub fn set_status(mut self, status: FarmStatus) -> Self {
        self.status = status;
        self.updated_at = OffsetDateTime::now_local()
            .expect("Failed to get current time");
        self
    }

    /// Convert Farm to API response DTO
    pub fn to_dto(&self) -> FarmDto {
        FarmDto {
            id: self.id,
            producer_id: self.producer_id,
            name: self.name.clone(),
            location_description: self.location_description.clone(),
            farm_type: self.farm_type.clone(),
            status: self.status,
        }
    }
}

/// Farm DTO for API responses
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FarmDto {
        /// Unique identifier
        pub id: Uuid,
        /// Producer ID
        pub producer_id: uuid::Uuid,
        /// Farm name
        pub name: String,
        /// Location description
        pub location_description: String,
        /// Farm type
        pub farm_type: String,
        /// Farm status
        pub status: FarmStatus,
    }
}