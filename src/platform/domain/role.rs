//! Role and Permission entities.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: Uuid,
    pub code: String,
    pub description: Option<String>,
}

/// Canonical role codes from the constitution (§239-244).
pub const CANONICAL_ROLES: [&str; 4] = ["admin", "producer", "customer", "support"];
pub const DEFAULT_REGISTRATION_ROLE: &str = "customer";
