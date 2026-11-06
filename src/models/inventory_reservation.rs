// InventoryReservation model per data-model.md Commerce Domain entity
// Fields: id, inventory_item_id, order_id, quantity, status, expires_at, idempotency_key
// Constraints: ck_reservation_quantity CHECK (quantity > 0)
//             idempotency_key UNIQUE across all reservations

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;
use numeric::Numeric;

/// InventoryReservation platform entity
/// Owned by: Commerce Domain (constitution.md §127-137)
/// Responsible for: Stock reservation by order with idempotency (constitution.md §127-137)
/// Relationships:
///   - MANY-TO-ONE to InventoryItem (inventory_item_id → commerce.inventory_items.id)
///   - MANY-TO-ONE to Order (order_id → "order".orders.id)

/// Reservation status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReservationStatus {
    Pending,      // Reservation created, not yet confirmed
    Confirmed,    // Reservation confirmed, inventory held
    Cancelled,    // Reservation cancelled, inventory released
    Expired,      // Reservation expired, inventory released
}

/// InventoryReservation platform entity
/// Fields: id, inventory_item_id, order_id, quantity, status, expires_at, idempotency_key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReservation {
    /// Unique identifier
    pub id: uuid::Uuid,
    /// Inventory item reference (product + batch + location)
    pub inventory_item_id: uuid::Uuid,
    /// Order reference (order domain)
    pub order_id: uuid::Uuid,
    /// Reservation quantity
    pub quantity: numeric::Numeric,
    /// Reservation status
    pub status: ReservationStatus,
    /// When the reservation expires (if not confirmed before this time)
    pub expires_at: Option<OffsetDateTime>,
    /// Idempotency key for safe retry
    pub idempotency_key: String,
    /// When the reservation was created
    pub created_at: OffsetDateTime,
    /// When the reservation was last updated
    pub updated_at: OffsetDateTime,
}

/// Reservation status checks
impl InventoryReservation {
    /// Check if reservation is active (pending or confirmed)
    pub fn is_active(&self) -> bool {
        matches!(self.status, ReservationStatus::Pending | ReservationStatus::Confirmed)
    }

    /// Check if reservation is expired
    pub fn is_expired(&self, current_time: Option<OffsetDateTime>) -> bool {
        match (&self.expires_at, current_time) {
            (Some(expires), Some(now)) => now > *expires,
            (None, _) => false, // No expiration set
            _ => false,
        }
    }

    /// Check if reservation can be cancelled
    pub fn can_cancel(&self) -> bool {
        self.status == ReservationStatus::Pending || self.status == ReservationStatus::Confirmed
    }

    /// Get remaining quantity that can still be reserved
    pub fn remaining_capacity(&self, total_quantity: numeric::Numeric) -> numeric::Numeric {
        numeric::Numeric::max(numeric::Numeric::from(0), total_quantity - self.quantity_reserved)
    }
}

/// InventoryReservation creation helper
impl InventoryReservation {
    /// Create a new reservation
    pub fn new(
        inventory_item_id: uuid::Uuid,
        order_id: uuid::Uuid,
        quantity: numeric::Numeric,
    ) -> Self {
        let now = OffsetDateTime::now_local()
            .expect("Failed to get current time");
        let idempotency_key = format!("res-{}-{}", uuid::Uuid::new_v4(), now.format("%Y%m%d%H%M%S%.3f"));
        
        InventoryReservation {
            id: uuid::Uuid::new_v4(),
            inventory_item_id,
            order_id,
            quantity,
            status: ReservationStatus::Pending,
            expires_at: None,
            idempotency_key,
            created_at: now,
            updated_at: now,
        }
    }

    /// Confirm a reservation (transition Pending → Confirmed)
    pub fn confirm(&mut self) {
        self.status = ReservationStatus::Confirmed;
        self.updated_at = OffsetDateTime::now_local()
            .expect("Failed to get current time");
    }

    /// Cancel a reservation (transition any → Cancelled)
    pub fn cancel(&mut self) {
        self.status = ReservationStatus::Cancelled;
        self.updated_at = OffsetDateTime::now_local()
            .expect("Failed to get current time");
    }

    /// Expire a reservation (transition Pending/Confirmed → Expired)
    pub fn expire(&mut self, current_time: OffsetDateTime) {
        if self.is_expired(current_time) {
            self.status = ReservationStatus::Expired;
            self.updated_at = current_time;
        }
    }

    /// Convert InventoryReservation to API response DTO (no sensitive data)
    pub fn to_dto(&self) -> InventoryReservationDto {
        InventoryReservationDto {
            id: self.id,
            inventory_item_id: self.inventory_item_id,
            order_id: self.order_id,
            quantity: self.quantity.to_string(),
            status: format!("{:?}", self.status),
            expires_at: self.expires_at.map(|t| t.to_string()).unwrap_or_default(),
            idempotency_key: self.idempotency_key.clone(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

/// InventoryReservation DTO for API responses
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InventoryReservationDto {
        /// Unique identifier
        pub id: uuid::Uuid,
        /// Inventory item ID
        pub inventory_item_id: uuid::Uuid,
        /// Order ID
        pub order_id: uuid::Uuid,
        /// Reservation quantity
        pub quantity: String,
        /// Reservation status
        pub status: String,
        /// Expiration date (if any)
        pub expires_at: String,
        /// Idempotency key
        pub idempotency_key: String,
        /// Created at timestamp
        pub created_at: String,
        /// Updated at timestamp
        pub updated_at: String,
    }
}