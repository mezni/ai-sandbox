// Brand model per data-model.md Commerce Domain entity
// Fields: id, name, description, status, created_at
// Relationships: MANY-TO-ONE inverse: commerce.products.brand_id → brands.id (optional)

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Brand platform entity
/// Owned by: Commerce Domain (constitution.md §127-137)
/// Responsible for: Product branding and organization (constitution.md §127-137)

/// Brand status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrandStatus {
    Active,
    Inactive,
}

/// Brand platform entity
/// Fields: id, name, description, status, created_at
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brand {
    /// Unique identifier
    pub id: Uuid,
    /// Brand name
    pub name: String,
    /// Brand description
    pub description: String,
    /// Brand status
    pub status: BrandStatus,
    /// When the brand was created
    pub created_at: time::OffsetDateTime,
    /// When the brand was last updated
    pub updated_at: time::OffsetDateTime,
}

/// Brand status check
impl Brand {
    /// Check if brand is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, BrandStatus::Active)
    }

    /// Check if brand is inactive
    pub fn is_inactive(&self) -> bool {
        matches!(self.status, BrandStatus::Inactive)
    }
}

/// Brand creation helper
impl Brand {
    /// Create a new brand with Active status
    pub fn new(name: String, description: String) -> Self {
        let now = time::OffsetDateTime::now_local()
            .expect("Failed to get current time");
        Brand {
            id: uuid::Uuid::new_v4(),
            name,
            description,
            status: BrandStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }

    /// Convert Brand to API response DTO
    pub fn to_dto(&self) -> BrandDto {
        BrandDto {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            is_active: self.is_active(),
        }
    }
}

/// Brand DTO for API responses
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BrandDto {
        /// Unique identifier
        pub id: Uuid,
        /// Brand name
        pub name: String,
        /// Brand description
        pub description: String,
        /// Whether brand is active
        pub is_active: bool,
    }
}