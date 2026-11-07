//! Audit event value object (append-only, never contains secrets).

use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub actor_id: Option<Uuid>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    pub metadata: Value,
    pub correlation_id: Option<Uuid>,
}

impl AuditEntry {
    pub fn new(action: impl Into<String>, resource_type: impl Into<String>) -> Self {
        Self {
            actor_id: None,
            action: action.into(),
            resource_type: resource_type.into(),
            resource_id: None,
            metadata: Value::Object(serde_json::Map::new()),
            correlation_id: None,
        }
    }

    pub fn actor(mut self, actor_id: Uuid) -> Self {
        self.actor_id = Some(actor_id);
        self
    }

    pub fn resource(mut self, resource_id: Uuid) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    pub fn correlation(mut self, request_id: Option<Uuid>) -> Self {
        self.correlation_id = request_id;
        self
    }

    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = metadata;
        self
    }
}
