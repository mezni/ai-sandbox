// Product model per data-model.md Commerce Domain entity
// Fields: id, producer_id, category_id, brand_id, name, slug, description,
// product_type, pricing_model, unit_of_measure, traceability_required, status,
// created_at, updated_at

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use time::OffsetDateTime;

/// Product platform entity
/// Owned by: Commerce Domain (constitution.md §127-137)
/// Responsible for: Product catalog, pricing, inventory tracking (constitution.md §131-137)
/// Relationships:
///   - MANY-TO-ONE to Category (category_id → commerce.categories.id)
///   - MANY-TO-ONE to Brand (brand_id → commerce.brands.id) (optional)
///   - MANY-TO-ONE to Producer (producer_id → traceability.producers.id)
///   - Product publication requires application-level validation of producer status,
///     required certifications, and traceability conditions (database design §489-491)

/// Product status lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductStatus {
    /// Product is in development, not yet available for sale
    Draft,
    /// Product is available for purchase
    Active,
    /// Product is archived/removed from catalog
    Archived,
}

/// Product pricing model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PricingModel {
    /// Unit pricing (per piece, per kg, per liter)
    UnitPricing,
    /// Weight pricing (per kg, per lb)
    WeightPricing,
    /// Volume pricing (per L, per gal)
    VolumePricing,
}

/// Product platform entity
/// Fields: id, producer_id, category_id, brand_id, name, slug, description,
/// product_type, pricing_model, unit_of_measure, traceability_required, status,
/// created_at, updated_at
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    /// Unique identifier
    pub id: Uuid,
    /// Producer reference from Traceability context (producer_id NOT NULL)
    pub producer_id: Uuid,
    /// Category reference (category_id NOT NULL)
    pub category_id: Uuid,
    /// Brand reference (optional)
    pub brand_id: Option<Uuid>,
    /// Product name
    pub name: String,
    /// URL-friendly slug (unique per commerce context)
    pub slug: String,
    /// Product description
    pub description: String,
    /// Pricing model
    pub pricing_model: PricingModel,
    /// Unit of measure (kg, g, L, ml, unit, etc.)
    pub unit_of_measure: String,
    /// Product traceability requirement
    pub traceability_required: bool,
    /// Product status
    pub status: ProductStatus,
    /// When the product was created
    pub created_at: OffsetDateTime,
    /// When the product was last updated
    pub updated_at: OffsetDateTime,
}

/// Product status transitions
impl Product {
    /// Transition from draft to active (publish to catalog)
    pub fn activate(&mut self) {
        self.status = ProductStatus::Active;
        self.updated_at = OffsetDateTime::now_local()
            .expect("Failed to get current time");
    }

    /// Transition from active to archived (remove from catalog)
    pub fn archive(&mut self) {
        self.status = ProductStatus::Archived;
        self.updated_at = OffsetDateTime::now_local()
            .expect("Failed to get current time");
    }

    /// Check if product is available for purchase
    pub fn is_available(&self) -> bool {
        self.status == ProductStatus::Active && !self.is_archived()
    }

    /// Check if product is in draft status
    pub fn is_draft(&self) -> bool {
        self.status == ProductStatus::Draft
    }

    /// Check if product is archived
    pub fn is_archived(&self) -> bool {
        self.status == ProductStatus::Archived
    }

    /// Check if traceability is required for this product
    pub fn requires_traceability(&self) -> bool {
        self.traceability_required
    }
}

/// Convert Product to API response DTO (excluding sensitive references)
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ProductDto {
        /// Unique identifier
        pub id: Uuid,
        /// Product name
        pub name: String,
        /// Product slug
        pub slug: String,
        /// Product description
        pub description: String,
        /// Pricing model
        pub pricing_model: PricingModel,
        /// Unit of measure
        pub unit_of_measure: String,
        /// Product status
        pub status: ProductStatus,
        /// Product is available for purchase
        pub available: bool,
        /// Product requires traceability
        pub traceability_required: bool,
    }
}

impl Product {
    /// Create a new product with draft status
    pub fn new(
        producer_id: Uuid,
        category_id: Uuid,
        name: String,
        slug: String,
        description: String,
        pricing_model: PricingModel,
        unit_of_measure: String,
        traceability_required: bool,
    ) -> Self {
        let now = OffsetDateTime::now_local()
            .expect("Failed to get current time");
        Product {
            id: uuid::Uuid::new_v4(),
            producer_id,
            category_id,
            brand_id: None,
            name,
            slug,
            description,
            pricing_model,
            unit_of_measure,
            traceability_required,
            status: ProductStatus::Draft,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set the brand association
    pub fn with_brand(mut self, brand_id: Uuid) -> Self {
        self.brand_id = Some(brand_id);
        self
    }
}