//! SQLx implementation of the role repository.

use async_trait::async_trait;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::platform::application::ports::RoleRepository;
use crate::platform::domain::errors::PlatformError;
use crate::platform::domain::role::Role;

use super::internal;

#[derive(sqlx::FromRow)]
struct RoleRow {
    id: Uuid,
    code: String,
    name: String,
    description: Option<String>,
    created_at: OffsetDateTime,
}

impl From<RoleRow> for Role {
    fn from(row: RoleRow) -> Self {
        Self {
            id: row.id,
            code: row.code,
            name: row.name,
            description: row.description,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SqlxRoleRepository {
    pool: PgPool,
}

impl SqlxRoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoleRepository for SqlxRoleRepository {
    async fn find_by_code(&self, code: &str) -> Result<Option<Role>, PlatformError> {
        let row = sqlx::query_as::<_, RoleRow>(
            "SELECT id, code, name, description, created_at \
             FROM platform.roles WHERE code = $1",
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(internal)?;

        Ok(row.map(Role::from))
    }

    async fn codes_for_user(&self, user_id: Uuid) -> Result<Vec<String>, PlatformError> {
        let rows: Vec<(String,)> = sqlx::query_as(
            "SELECT r.code FROM platform.roles r \
             JOIN platform.user_roles ur ON ur.role_id = r.id \
             WHERE ur.user_id = $1 ORDER BY r.code",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(internal)?;

        Ok(rows.into_iter().map(|(code,)| code).collect())
    }

    async fn assign_role(&self, user_id: Uuid, role_code: &str) -> Result<(), PlatformError> {
        let role = self
            .find_by_code(role_code)
            .await?
            .ok_or(PlatformError::NotFound)?;

        sqlx::query(
            "INSERT INTO platform.user_roles (user_id, role_id) VALUES ($1, $2) \
             ON CONFLICT (user_id, role_id) DO NOTHING",
        )
        .bind(user_id)
        .bind(role.id)
        .execute(&self.pool)
        .await
        .map_err(internal)?;

        Ok(())
    }
}
