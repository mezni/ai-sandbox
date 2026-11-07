//! SQLx implementation of the user repository.

use async_trait::async_trait;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::platform::application::ports::UserRepository;
use crate::platform::domain::errors::PlatformError;
use crate::platform::domain::user::{User, UserStatus};

use super::{internal, is_unique_violation};

#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    password_hash: String,
    status: String,
    email_verified_at: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl TryFrom<UserRow> for User {
    type Error = PlatformError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            email: row.email,
            password_hash: row.password_hash,
            status: UserStatus::from_db(&row.status)?,
            email_verified_at: row.email_verified_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

const COLUMNS: &str = "id, email, password_hash, status, email_verified_at, created_at, updated_at";

#[derive(Debug, Clone)]
pub struct SqlxUserRepository {
    pool: PgPool,
}

impl SqlxUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, PlatformError> {
        let query = format!("SELECT {COLUMNS} FROM platform.users WHERE lower(email) = lower($1)");
        let row = sqlx::query_as::<_, UserRow>(&query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(internal)?;

        row.map(User::try_from).transpose()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, PlatformError> {
        let query = format!("SELECT {COLUMNS} FROM platform.users WHERE id = $1");
        let row = sqlx::query_as::<_, UserRow>(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(internal)?;

        row.map(User::try_from).transpose()
    }

    async fn create(&self, user: &User) -> Result<User, PlatformError> {
        let query = format!(
            "INSERT INTO platform.users \
             (id, email, password_hash, status, email_verified_at, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) \
             RETURNING {COLUMNS}"
        );
        let row = sqlx::query_as::<_, UserRow>(&query)
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(user.status.as_str())
            .bind(user.email_verified_at)
            .bind(user.created_at)
            .bind(user.updated_at)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                if is_unique_violation(&err) {
                    PlatformError::EmailAlreadyExists
                } else {
                    internal(err)
                }
            })?;

        User::try_from(row)
    }

    async fn update_email(&self, id: Uuid, email: &str) -> Result<User, PlatformError> {
        let query = format!(
            "UPDATE platform.users SET email = $2, updated_at = NOW() \
             WHERE id = $1 RETURNING {COLUMNS}"
        );
        let row = sqlx::query_as::<_, UserRow>(&query)
            .bind(id)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| {
                if is_unique_violation(&err) {
                    PlatformError::EmailAlreadyExists
                } else {
                    internal(err)
                }
            })?
            .ok_or(PlatformError::NotFound)?;

        User::try_from(row)
    }
}
