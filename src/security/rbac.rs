// RBAC middleware per docs/06-api-specification.md §4.2-4.3 and constitution.md §52-59
// Roles: CUSTOMER, FARMER, SELLER, ADMIN

use axum::{
    extract::{FromRequestParts, Request, State},
    http::{Request, StatusCode},
    middleware::{self, Next},
    Json,
};
use serde::Deserialize;
use serde::Serialize;
use traceability_domain::domain::role::Role; // hypothetical import

/// RBAC configuration with role definitions
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RbacConfig {
    /// Allowed roles for the application
    pub roles: Vec<RoleConfig>,
}

/// Role configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoleConfig {
    /// Role code (CUSTOMER, FARMER, SELLER, ADMIN)
    pub code: String,
    /// Role name
    pub name: String,
    /// Description of the role
    pub description: String,
    /// Whether this role is administrative
    pub is_admin: bool,
}

/// Pre-defined role configurations per API specification §4.2-4.3
pub const RBAC_CONFIG: RbacConfig = RbacConfig {
    roles: vec![
        RoleConfig {
            code: "CUSTOMER".to_string(),
            name: "Customer".to_string(),
            description: "Customer role - can purchase products, manage own orders".to_string(),
            is_admin: false,
        },
        RoleConfig {
            code: "FARMER".to_string(),
            name: "Farmer".to_string(),
            description: "Farmer role - can manage farms, certifications, batches".to_string(),
            is_admin: false,
        },
        RoleConfig {
            code: "SELLER".to_string(),
            name: "Seller".to_string(),
            description: "Seller role - can manage products, inventory, listings".to_string(),
            is_admin: false,
        },
        RoleConfig {
            code: "ADMIN".to_string(),
            name: "Administrator".to_string(),
            description: "Administrator role - can manage platform operations".to_string(),
            is_admin: true,
        },
    ],
];

/// Extract the authenticated user from the request state
pub async fn extract_user(
    State(rbac_config): State<RbacConfig>,
    parts: &mut axum::extract::RequestParts,
) -> Result<RoleConfig, StatusCode> {
    // Extract user from JWT token in the request
    // This is a simplified implementation - in practice, the user would be
    // attached to the request state by the auth middleware
    
    // For now, return the first role as a default
    // TODO: Replace with actual user extraction from authentication middleware
    RBAC_CONFIG
        .roles
        .first()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
}

/// Check if a user with a given role has permission to access an endpoint
pub async fn check_permission(
    user_role: &RoleConfig,
    required_role: &RoleConfig,
) -> bool {
    // Admin can access everything
    if user_role.is_admin || required_role.is_admin {
        return true;
    }
    
    // Check role hierarchy: FARMER can access some SELLER endpoints vice versa
    // Simple equality check for now
    user_role.code == required_role.code
}

/// Middleware that enforces RBAC on protected endpoints
pub async fn rbac_middleware(
    State(rbac_config): State<RbacConfig>,
    mut request: Request,
    next: Next,
) -> Result<(axum::http::Response, String), StatusCode> {
    // Extract user from request
    let user_role = extract_user(&rbac_config, &mut request)
        .await
        .map_err(|e| StatusCode::FORBIDDEN)?;
    
    // Determine what role is required for this endpoint
    // In a real implementation, this would be derived from the route annotations
    let required_role = &RBAC_CONFIG.roles[0]; // Default to first role
    
    // Check permission
    if !check_permission(&user_role, required_role) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Continue to the next handler
    let response = next.run(request).await;
    Ok((response, String::new()))
}

/// Role hierarchy for more fine-grained access control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoleLevel {
    Customer = 1,
    Farmer = 2,
    Seller = 3,
    Admin = 4,
}

/// Check if a role can perform an action based on role level
pub fn can_action(role_level: RoleLevel, required_level: RoleLevel) -> bool {
    role_level as u8 >= required_level as u8
}

/// Role enum per constitution.md approved domains
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    Customer,
    Farmer,
    Seller,
    Admin,
}

/// Convert RoleConfig to Role enum
impl From<&RoleConfig> for Role {
    fn from(role: &RoleConfig) -> Self {
        match role.code.as_str() {
            "CUSTOMER" => Role::Customer,
            "FARMER" => Role::Farmer,
            "SELLER" => Role::Seller,
            "ADMIN" => Role::Admin,
            _ => Role::Customer,
        }
    }
}

/// Convert Role to string
impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Customer => "CUSTOMER",
            Role::Farmer => "FARMER",
            Role::Seller => "SELLER",
            Role::Admin => "ADMIN",
        }
        .to_string()
    }
}

/// Convert &str to Role
impl TryFrom<&str> for Role {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "CUSTOMER" => Ok(Role::Customer),
            "FARMER" => Ok(Role::Farmer),
            "SELLER" => Ok(Role::Seller),
            "ADMIN" => Ok(Role::Admin),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_hierarchy() {
        assert!(can_action(RoleLevel::Admin, RoleLevel::Customer));
        assert!(can_action(RoleLevel::Admin, RoleLevel::Farmer));
        assert!(can_action(RoleLevel::Admin, RoleLevel::Seller));
        assert!(can_action(RoleLevel::Seller, RoleLevel::Customer));
        assert!(can_action(RoleLevel::Farmer, RoleLevel::Customer));
        assert!(!can_action(RoleLevel::Customer, RoleLevel::Farmer));
    }

    #[test]
    fn test_role_conversion() {
        let config_role = &RBAC_CONFIG.roles[0]; // CUSTOMER
        let role: Role = config_role.into();
        assert_eq!(role, Role::Customer);
        assert_eq!(role.to_string(), "CUSTOMER");
        
        let result: Result<Role, String> = Role::try_from("ADMIN");
        assert_eq?(result, Role::Admin);
    }
}