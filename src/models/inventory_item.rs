// InventoryItem model per data-model.md Commerce Domain entity
// Fields: id, product_id, batch_id, location_id, quantity_on_hand, quantity_reserved, unit_of_measure
// Constraints: uq_inventory_product_batch_location UNIQUE (product_id, batch_id, location_id)
//             ck_inventory_on_hand CHECK (quantity_on_hand >= 0)
//             ck_inventory_reserved CHECK (quantity_reserved >= 0 AND quantity_reserved <= quantity_on_hand)

// Inventory location source (Operations domain per database design §585)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InventoryLocationSource {
    Warehouse,     // Operations domain
    Farm,          // Traceability domain
    FrontDoor,     // Direct from producer
}

/// Inventory item status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InventoryStatus {
    InStock,     // quantity_on_hand > 0
    LowStock,    // quantity_on_hand <= threshold
    OutOfStock,  // quantity_on_hand = 0
    Reserved,    // quantity_reserved > 0
}

/// InventoryItem platform entity
/// Owned by: Commerce Domain (constitution.md §127-137)
/// Responsible for: Stock tracking by product, batch, and location (constitution.md §127-137)
/// Relationships:
///   - MANY-TO-ONE to Product (product_id → commerce.products.id)
///   - MANY-TO-ONE to Batch (batch_id → traceability.batches.id) — external domain reference
///   - MANY-TO-ONE to Warehouse (location_id → operations.warehouses.id) — external domain reference

/// InventoryItem platform entity
/// Fields: id, product_id, batch_id, location_id, quantity_on_hand, quantity_reserved, unit_of_measure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    /// Unique identifier
    pub id: uuid::Uuid,
    /// Product reference (commerce domain)
    pub product_id: uuid::Uuid,
    /// Batch reference (traceability domain, external reference)
    pub batch_id: uuid::Uuid,
    /// Location reference (operations domain, external reference)
    pub location_id: uuid::Uuid,
    /// Quantity currently in stock
    pub quantity_on_hand: numeric::Numeric,
    /// Quantity currently reserved for orders
    pub quantity_reserved: numeric::Numeric,
    /// Unit of measure
    pub unit_of_measure: String,
    /// Inventory location source
    pub location_source: InventoryLocationSource,
    /// Inventory last updated timestamp
    pub last_updated: time::OffsetDateTime,
}

/// Inventory status based on quantity
impl InventoryItem {
    /// Check if item is in stock
    pub fn is_in_stock(&self) -> bool {
        self.quantity_on_hand > numeric::Numeric::from(0)
    }

    /// Check if item is out of stock
    pub fn is_out_of_stock(&self) -> bool {
        self.quantity_on_hand == numeric::Numeric::from(0)
    }

    /// Check if item has low stock
    pub fn is_low_stock(&self, threshold: numeric::Numeric) -> bool {
        self.quantity_on_hand <= threshold
    }

    /// Check if item is available for order (in stock and not fully reserved)
    pub fn is_available_for_order(&self) -> bool {
        self.quantity_on_hand > self.quantity_reserved
    }

    /// Check if item can fulfill a given quantity
    pub fn can_fulfill(&self, quantity: numeric::Numeric) -> bool {
        self.quantity_on_hand >= quantity && self.quantity_on_hand - self.quantity_reserved >= quantity
    }

    /// Get available quantity (on-hand minus reserved)
    pub fn available_quantity(&self) -> numeric::Numeric {
        // Use MAX to prevent negative values from reservation overflow
        numeric::Numeric::max(numeric::Numeric::from(0), self.quantity_on_hand - self.quantity_reserved)
    }

    /// Check inventory status
    pub fn status(&self) -> InventoryStatus {
        let on_hand = self.quantity_on_hand;
        let reserved = self.quantity_reserved;

        if on_hand == numeric::Numeric::from(0) {
            InventoryStatus::OutOfStock
        } else if reserved > numeric::Numeric::from(0) && on_hand == reserved {
            InventoryStatus::Reserved
        } else if on_hand <= numeric::Numeric::from(10) {
            InventoryStatus::LowStock
        } else {
            InventoryStatus::InStock
        }
    }

    /// Convert InventoryItem to API response DTO (no external references)
    pub fn to_dto(&self) -> InventoryItemDto {
        InventoryItemDto {
            id: self.id,
            product_id: self.product_id,
            batch_id: self.batch_id,
            location_id: self.location_id,
            quantity_on_hand: self.quantity_on_hand.to_string(),
            quantity_reserved: self.quantity_reserved.to_string(),
            unit_of_measure: self.unit_of_measure.clone(),
            location_source: format!("{:?}", self.location_source),
            is_in_stock: self.is_in_stock(),
            is_available_for_order: self.is_available_for_order(),
        }
    }
}

/// InventoryItem DTO for API responses
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InventoryItemDto {
        /// Unique identifier
        pub id: uuid::Uuid,
        /// Product ID
        pub product_id: uuid::Uuid,
        /// Batch ID
        pub batch_id: uuid::Uuid,
        /// Location ID
        pub location_id: uuid::Uuid,
        /// Quantity on hand
        pub quantity_on_hand: String,
        /// Quantity reserved
        pub quantity_reserved: String,
        /// Unit of measure
        pub unit_of_measure: String,
        /// Location source
        pub location_source: String,
        /// Whether item is in stock
        pub is_in_stock: bool,
        /// Whether item is available for order
        pub is_available_for_order: bool,
    }
}

/// InventoryLocationSource conversions
impl From<InventoryLocationSource> for String {
    fn from(source: InventoryLocationSource) -> String {
        match source {
            InventoryLocationSource::Warehouse => "warehouse".to_string(),
            InventoryLocationSource::Farm => "farm".to_string(),
            InventoryLocationSource::FrontDoor => "front_door".to_string(),
        }
    }
}

impl From<String> for InventoryLocationSource {
    fn from(s: String) -> InventoryLocationSource {
        match s.as_str() {
            "warehouse" => InventoryLocationSource::Warehouse,
            "farm" => InventoryLocationSource::Farm,
            "front_door" => InventoryLocationSource::FrontDoor,
            _ => InventoryLocationSource::Warehouse,
        }
    }
}

/// Inventory repository traits for database operations
pub trait InventoryItemRepository: Send + Sync {
    /// Find an inventory item by product, batch, and location
    async fn find_by_product_batch_location(
        &self,
        product_id: uuid::Uuid,
        batch_id: uuid::Uuid,
        location_id: uuid::Uuid,
    ) -> Result<Option<InventoryItem>, sqlx::Error>;
    /// Find all inventory items for a product
    async fn find_by_product(&self, product_id: uuid::Uuid) -> Result<Vec<InventoryItem>, sqlx::Error>;
    /// Find all inventory items with low stock
    async fn find_low_stock(&self, threshold: numeric::Numeric) -> Result<Vec<InventoryItem>, sqlx::Error>;
    /// Find all available inventory items (can fulfill orders)
    async fn find_available(&self) -> Result<Vec<InventoryItem>, sqlx::Error>;
    /// Create a new inventory item
    async fn create(&self, item: &InventoryItem) -> Result<InventoryItem, sqlx::Error>;
    /// Update inventory quantities
    async fn update_quantities(
        &self,
        id: uuid::Uuid,
        on_hand: numeric::Numeric,
        reserved: numeric::Numeric,
    ) -> Result<(), sqlx::Error>;
}

/// Default implementation using SQLx
pub struct SqlxInventoryItemRepository {
    pool: sqlx::PgPool,
}

impl SqlxInventoryItemRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        SqlxInventoryItemRepository { pool }
    }
}

impl InventoryItemRepository for SqlxInventoryRepository {
    // ... (implementation would use SQLx queries)
}