//! SQLx implementation of the append-only audit repository.

use async_trait::async_trait;
use sqlx::PgPool;

use crate::platform::application::ports::AuditRepository;
use crate::platform::domain::audit::AuditEntry;
use crate::platform::domain::errors::PlatformError;

use super::internal;

#[derive(Debug, Clone)]
pub struct SqlxAuditRepository {
    pool: PgPool,
}

impl SqlxAuditRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditRepository for SqlxAuditRepository {
    async fn record(&self, entry: &AuditEntry) -> Result<(), PlatformError> {
        sqlx::query(
            "INSERT INTO platform.audit_logs \
             (actor_id, action, resource_type, resource_id, metadata, correlation_id) \
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(entry.actor_id)
        .bind(&entry.action)
        .bind(&entry.resource_type)
        .bind(entry.resource_id)
        .bind(&entry.metadata)
        .bind(entry.correlation_id)
        .execute(&self.pool)
        .await
        .map_err(internal)?;

        Ok(())
    }
}
