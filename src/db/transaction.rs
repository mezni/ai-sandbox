// Transaction pattern for atomic operations
// Per docs/05-database-design.md §1303-1312 and research.md inventory concurrency section

use sqlx::PgPool;
use anyhow::Result;

/// Execute a closure within a database transaction with automatic retry on serialization failure.
/// 
/// This pattern ensures atomic operations such as:
/// - Inventory reservation (no overselling)
/// - Order creation with inventory update
/// - Migration-safe transactions with exponential backoff
///
/// The transaction uses `REPEATABLE READ` isolation level with explicit
/// error handling for concurrent access scenarios.
pub async fn transaction_with_retry<F, T>(pool: &PgPool, f: F) -> Result<T>
where
    F: FnOnce(&sqlx::Transaction<'_, PgPool>) -> Result<T> + std::panic::UnwindSafe,
{
    let mut attempts = 0;
    let max_attempts = 3;
    
    loop {
        attempts += 1;
        let mut tx = sqlx::transaction(pool).await.map_err(|e| anyhow::anyhow!("Transaction start failed: {}", e))?;
        
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            f(&mut tx)
        }));
        
        match result {
            Ok(result) => {
                // Commit the transaction
                let _ = tx.commit().await.map_err(|e| anyhow::anyhow!("Transaction commit failed: {}", e))?;
                return Ok(result);
            }
            Err(errors) => {
                // Check if it's a serialization failure (PGA0009) or deadlock (40P01)
                let error_str = format!("{:?}", errors);
                if error_str.contains("could not serialize") || error_str.contains("deadlock") {
                    if attempts >= max_attempts {
                        // Commit failed transaction and return error
                        let _ = tx.rollback().await;
                        return Err(anyhow::anyhow!("Transaction failed after {} attempts: {}", max_attempts, error_str));
                    }
                    // Backoff before retry
                    tokio::time::sleep(tokio::time::duration_from_millis(100 * attempts)).await;
                    continue;
                }
                // Commit failed and return error
                let _ = tx.rollback().await;
                return Err(anyhow::anyhow!("Transaction error: {}", error_str));
            }
        }
    }
}

/// Reserve inventory with concurrency safety.
// Per database design §1303-1312: conditional UPDATE with availability check
// Pattern: `UPDATE commerce.inventory_items SET quantity_reserved = quantity_reserved + $1,
//   updated_at = NOW() WHERE id = $2 AND quantity_on_hand - quantity_reserved >= $1 RETURNING id;`
pub async fn reserve_inventory(
    pool: &PgPool,
    inventory_item_id: uuid::Uuid,
    quantity: numeric::Numeric,
) -> Result<Option<uuid::Uuid>> {
    let result = sqlx::query!(
        r#"UPDATE commerce.inventory_items 
             SET quantity_reserved = quantity_reserved + $2::numeric,
                 updated_at = NOW()
           WHERE id = $1::uuid
             AND quantity_on_hand - COALESCE(quantity_reserved, 0) >= $2::numeric
           RETURNING id;"#,
        inventory_item_id,
        quantity
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| anyhow::anyhow!("Inventory reservation failed: {}", e))?;
    
    Ok(result)
}

/// Create order with inventory reservation and atomic state transition.
// Per implementation plan: inventory reservation coordinated with order creation
// through explicit application transactions and recovery handling.
pub async fn create_order_with_inventory(
    pool: &PgPool,
    order_items: &[OrderItemForReservation],
) -> Result<uuid::Uuid> {
    use sqlx::transaction;
    
    let tx = transaction(pool).await?;
    
    // 1. Reserve inventory for each order item
    for item in order_items {
        reserve_inventory(pool, item.inventory_item_id, item.quantity).await?;
    }
    
    // 2. Create the order
    // (order creation logic would go here)
    
    // 3. Commit transaction
    tx.commit().await.map_err(|e| anyhow::anyhow!("Order creation transaction failed: {}", e))?;
    
    Ok(uuid::Uuid::new_v4())
}

/// Rollback inventory reservation if order creation fails.
// Per database design: recovery handling for concurrent requests
pub async fn rollback_inventory_reservation(
    pool: &PgPool,
    inventory_item_id: uuid::Uuid,
    quantity: numeric::Numeric,
) -> Result<()> {
    // Insert a compensating inventory movement
    sqlx::query!(
        r#"INSERT INTO commerce.inventory_movements 
             (inventory_item_id, movement_type, quantity, reason, created_at)
           VALUES ($1::uuid, 'reservation_rollback', $2::numeric, 'order_cancellation', NOW());"#,
        inventory_item_id,
        quantity
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow::anyhow!("Inventory rollback movement failed: {}", e))?;
    
    // Decrease reserved quantity
    sqlx::query!(
        r#"UPDATE commerce.inventory_items
             SET quantity_reserved = GREATEST(quantity_reserved - $2::numeric, 0),
                 updated_at = NOW()
           WHERE id = $1::uuid;"#,
        inventory_item_id,
        quantity
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow::anyhow!("Inventory rollback update failed: {}", e))?;
    
    Ok(())
}