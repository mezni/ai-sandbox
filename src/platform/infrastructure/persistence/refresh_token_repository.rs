//! SQLx implementation of the refresh-token repository.

use async_trait::async_trait;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::platform::application::ports::RefreshTokenRepository;
use crate::platform::domain::errors::PlatformError;
use crate::platform::domain::refresh_token::RefreshToken;

use super::internal;

#[derive(sqlx::FromRow)]
struct RefreshTokenRow {
    id: Uuid,
    user_id: Uuid,
    token_hash: String,
    expires_at: OffsetDateTime,
    revoked_at: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
}

impl From<RefreshTokenRow> for RefreshToken {
    fn from(row: RefreshTokenRow) -> Self {
        Self {
            id: row.id,
            user_id: row.user_id,
            token_hash: row.token_hash,
            expires_at: row.expires_at,
            revoked_at: row.revoked_at,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SqlxRefreshTokenRepository {
    pool: PgPool,
}

impl SqlxRefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenRepository for SqlxRefreshTokenRepository {
    async fn insert(&self, token: &RefreshToken) -> Result<(), PlatformError> {
        sqlx::query(
            "INSERT INTO platform.refresh_tokens \
             (id, user_id, token_hash, expires_at, revoked_at, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(token.id)
        .bind(token.user_id)
        .bind(&token.token_hash)
        .bind(token.expires_at)
        .bind(token.revoked_at)
        .bind(token.created_at)
        .execute(&self.pool)
        .await
        .map_err(internal)?;

        Ok(())
    }

    async fn find_active_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<RefreshToken>, PlatformError> {
        let row = sqlx::query_as::<_, RefreshTokenRow>(
            "SELECT id, user_id, token_hash, expires_at, revoked_at, created_at \
             FROM platform.refresh_tokens \
             WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > NOW()",
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(internal)?;

        Ok(row.map(RefreshToken::from))
    }

    async fn rotate(&self, old_id: Uuid, new_token: &RefreshToken) -> Result<(), PlatformError> {
        let mut tx = self.pool.begin().await.map_err(internal)?;

        let revoked = sqlx::query(
            "UPDATE platform.refresh_tokens SET revoked_at = NOW() \
             WHERE id = $1 AND revoked_at IS NULL AND expires_at > NOW()",
        )
        .bind(old_id)
        .execute(&mut *tx)
        .await
        .map_err(internal)?;

        if revoked.rows_affected() == 0 {
            tx.rollback().await.ok();
            return Err(PlatformError::Unauthorized);
        }

        sqlx::query(
            "INSERT INTO platform.refresh_tokens \
             (id, user_id, token_hash, expires_at, revoked_at, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(new_token.id)
        .bind(new_token.user_id)
        .bind(&new_token.token_hash)
        .bind(new_token.expires_at)
        .bind(new_token.revoked_at)
        .bind(new_token.created_at)
        .execute(&mut *tx)
        .await
        .map_err(internal)?;

        tx.commit().await.map_err(internal)?;
        Ok(())
    }

    async fn revoke_by_hash_for_user(
        &self,
        token_hash: &str,
        user_id: Uuid,
    ) -> Result<(), PlatformError> {
        sqlx::query(
            "UPDATE platform.refresh_tokens SET revoked_at = NOW() \
             WHERE token_hash = $1 AND user_id = $2 AND revoked_at IS NULL",
        )
        .bind(token_hash)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(internal)?;

        Ok(())
    }
}
