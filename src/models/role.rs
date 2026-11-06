// Role model per data-model.md Platform Domain entity
// Fields: id, code, name, description, created_at
// Relationships: M:N with Permission through role_permissions junction
// M:N with User through user_roles junction

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

/// Role platform entity
/// Owned by: Platform Domain (constitution.md §99-116)
/// Responsible for: RBAC, authorization, access control (constitution.md §236-244)
/// Relationships:
///   - M:N with Permission through role_permissions junction
///   - 1:N with User through user_roles junction (role_id references platform.roles(id))

/// Role status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoleStatus {
    Active,
    Inactive,
}

/// Role platform entity
/// Fields: id, code, name, description, created_at, status
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    /// Unique identifier
    pub id: Uuid,
    /// Role code (CUSTOMER, FARMER, SELLER, ADMIN)
    pub code: String,
    /// Role name
    pub name: String,
    /// Role description
    pub description: String,
    /// Role status
    pub status: RoleStatus,
    /// When the role was created
    pub created_at: time::OffsetDateTime,
    /// When the role was last updated
    pub updated_at: time::OffsetDateTime,
}

/// Role creation helper
impl Role {
    /// Create a new role with Active status
    pub fn new(code: String, name: String, description: String) -> Self {
        let now = time::OffsetDateTime::now_local()
            .expect("Failed to get current time");
        Role {
            id: uuid::Uuid::new_v4(),
            code,
            name,
            description,
            status: RoleStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }

    /// Convert Role to RoleConfig for RBAC middleware
    pub fn to_config(&self) -> rbac::RoleConfig {
        rbac::RoleConfig {
            code: self.code.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            is_admin: matches!(self.code.as_str(), "ADMIN"),
        }
    }
}

/// Role repository traits for database operations
pub trait RoleRepository: Send + Sync {
    /// Find a role by its code
    async fn find_by_code(&self, code: &str) -> Result<Option<Role>, sqlx::Error>;
    /// Find all active roles
    async fn find_all_active(&self) -> Result<Vec<Role>, sqlx::Error>;
    /// Create a new role
    async fn create(&self, role: &Role) -> Result<Role, sqlx::Error>;
    /// Update a role's status
    async fn update_status(&self, id: uuid::Uuid, status: RoleStatus) -> Result<(), sqlx::Error>;
    /// Delete a role (soft delete via status change)
    async fn soft_delete(&self, id: uuid::Uuid) -> Result<(), sqlx::Error>;
}

/// Default implementation using SQLx
pub struct SqlxRoleRepository {
    pool: sqlx::PgPool,
}

impl SqlxRoleRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        SqlxRoleRepository { pool }
    }
}

impl RoleRepository for SqlxRoleRepository {
    async fn find_by_code(&self, code: &str) -> Result<Option<Role>, sqlx::Error> {
        let row = sqlx::query_as::<Role, _>(
            "SELECT id, code, name, description, status, created_at, updated_at FROM platform.roles WHERE code = $1"
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to find role by code: {}", e))?;

        Ok(row)
    }

    async fn find_all_active(&self) -> Result<Vec<Role>, sqlx::Error> {
        let rows = sqlx::query_as::<Role, _>(
            "SELECT id, code, name, description, status, created_at, updated_at FROM platform.roles WHERE status = 'Active' ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to find all active roles: {}", e))?;

        Ok(rows)
    }

    async fn create(&self, role: &Role) -> Result<Role, sqlx::Error> {
        let row = sqlx::query_as::<Role, _>(
            "INSERT INTO platform.roles (code, name, description, status, created_at, updated_at)
             VALUES ($1, $2, $3, 'Active', NOW(), NOW())
             RETURNING id, code, name, description, status, created_at, updated_at"
        )
        .bind(&role.code)
        .bind(&role.name)
        .bind(&role.description)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create role: {}", e))?;

        Ok(row)
    }

    async fn update_status(&self, id: uuid::Uuid, status: RoleStatus) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE platform.roles SET status = $2, updated_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .bind(status)
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to update role status: {}", e))?;
        Ok(())
    }

    async fn soft_delete(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        self.update_status(id, RoleStatus::Inactive).await
    }
}