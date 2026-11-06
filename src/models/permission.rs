// Permission model per data-model.md Platform Domain entity
// Fields: id, code, description
// Relationships: M:N with Role through role_permissions junction

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

/// Permission platform entity
/// Owned by: Platform Domain (constitution.md §99-116)
/// Responsible for: Individual permissions assigned to roles (constitution.md §99-116)
/// Relationships:
///   - M:N with Role through role_permissions junction (role_id, permission_id)
///   - No direct user assignment; enforced through role hierarchy

/// Permission entity
/// Fields: id, code, description
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Permission {
    /// Unique identifier
    pub id: Uuid,
    /// Permission code (e.g., "products.read", "orders.create")
    pub code: String,
    /// Permission description
    pub description: String,
}

/// Permission creation helper
impl Permission {
    /// Create a new permission
    pub fn new(code: String, description: String) -> Self {
        Permission {
            id: uuid::Uuid::new_v4(),
            code,
            description,
        }
    }

    /// Convert Permission to a string identifier
    pub fn identifier(&self) -> String {
        format!("{}:{}", self.namespace(), self.code)
    }

    /// Extract the namespace from the permission code
    /// (e.g., "products", "orders", "inventory", "auth" from "products.create", "orders.delete")
    fn namespace(&self) -> &str {
        // Take the part before the first dot, or the entire code if no dot
        if let Some(pos) = self.code.find('.') {
            &self.code[..pos]
        } else {
            &self.code
        }
    }

    /// Check if this permission applies to a given resource
    pub fn applies_to(&self, resource: &str) -> bool {
        self.namespace() == resource
    }
}

/// Permission repository traits for database operations
pub trait PermissionRepository: Send + Sync {
    /// Find a permission by its code
    async fn find_by_code(&self, code: &str) -> Result<Option<Permission>, sqlx::Error>;
    /// Find all permissions
    async fn find_all(&self) -> Result<Vec<Permission>, sqlx::Error>;
    /// Find permissions by namespace
    async fn find_by_namespace(&self, namespace: &str) -> Result<Vec<Permission>, sqlx::Error>;
    /// Create a new permission
    async fn create(&self, permission: &Permission) -> Result<Permission, sqlx::Error>;
}

/// Default implementation using SQLx
pub struct SqlxPermissionRepository {
    pool: sqlx::PgPool,
}

impl SqlxPermissionRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        SqlxPermissionRepository { pool }
    }
}

impl PermissionRepository for SqlxPermissionRepository {
    async fn find_by_code(&self, code: &str) -> Result<Option<Permission>, sqlx::Error> {
        let row = sqlx::query_as::<Permission, _>(
            "SELECT id, code, description FROM platform.permissions WHERE code = $1"
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to find permission by code: {}", e))?;

        Ok(row)
    }

    async fn find_all(&self) -> Result<Vec<Permission>, sqlx::Error> {
        let rows = sqlx::query_as::<Permission, _>(
            "SELECT id, code, description FROM platform.permissions ORDER BY code"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to find all permissions: {}", e))?;

        Ok(rows)
    }

    async fn find_by_namespace(&self, namespace: &str) -> Result<Vec<Permission>, sqlx::Error> {
        let rows = sqlx::query_as::<Permission, _>(
            "SELECT id, code, description FROM platform.permissions WHERE code LIKE $1 ORDER BY code"
        )
        .bind(format!("{}.%", namespace))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to find permissions by namespace: {}", e))?;

        Ok(rows)
    }

    async fn create(&self, permission: &Permission) -> Result<Permission, sqlx::Error> {
        let row = sqlx::query_as::<Permission, _>(
            "INSERT INTO platform.permissions (code, description) VALUES ($1, $2)
             RETURNING id, code, description"
        )
        .bind(&permission.code)
        .bind(&permission.description)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create permission: {}", e))?;

        Ok(row)
    }
}