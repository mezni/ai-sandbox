//! HS256 JWT access-token adapter.

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::platform::application::ports::{AccessClaims, TokenIssuer};
use crate::platform::domain::errors::PlatformError;
use crate::platform::domain::user::User;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    roles: Vec<String>,
    iss: String,
    aud: String,
    iat: i64,
    exp: i64,
}

#[derive(Debug, Clone)]
pub struct JwtIssuer {
    secret: String,
    access_ttl: i64,
    issuer: String,
    audience: String,
}

impl JwtIssuer {
    pub fn new(
        secret: impl Into<String>,
        access_ttl: i64,
        issuer: impl Into<String>,
        audience: impl Into<String>,
    ) -> Self {
        Self {
            secret: secret.into(),
            access_ttl,
            issuer: issuer.into(),
            audience: audience.into(),
        }
    }
}

impl TokenIssuer for JwtIssuer {
    fn issue_access(&self, user: &User, roles: &[String]) -> Result<(String, u64), PlatformError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let claims = Claims {
            sub: user.id,
            roles: roles.to_vec(),
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            iat: now,
            exp: now + self.access_ttl,
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|err| PlatformError::Internal(format!("failed to sign access token: {err}")))?;

        Ok((token, self.access_ttl as u64))
    }

    fn decode_access(&self, token: &str) -> Result<AccessClaims, PlatformError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[self.issuer.as_str()]);
        validation.set_audience(&[self.audience.as_str()]);

        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|_| PlatformError::Unauthorized)?;

        Ok(AccessClaims {
            sub: data.claims.sub,
            roles: data.claims.roles,
            iat: data.claims.iat,
            exp: data.claims.exp,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user() -> User {
        User::new("user@example.com".to_string(), "hash".to_string())
    }

    #[test]
    fn issues_and_decodes_access_tokens() {
        let issuer = JwtIssuer::new(
            "secret-secret-secret-secret-secret",
            900,
            "biosouk",
            "biosouk-users",
        );
        let user = user();
        let (token, ttl) = issuer
            .issue_access(&user, &["customer".to_string()])
            .unwrap();
        assert_eq!(ttl, 900);

        let claims = issuer.decode_access(&token).unwrap();
        assert_eq!(claims.sub, user.id);
        assert_eq!(claims.roles, vec!["customer".to_string()]);
    }

    #[test]
    fn rejects_tokens_signed_with_another_secret() {
        let issuer = JwtIssuer::new(
            "secret-secret-secret-secret-secret",
            900,
            "biosouk",
            "biosouk-users",
        );
        let other = JwtIssuer::new(
            "other-other-other-other-other-00",
            900,
            "biosouk",
            "biosouk-users",
        );
        let (token, _) = issuer
            .issue_access(&user(), &["customer".to_string()])
            .unwrap();
        assert!(other.decode_access(&token).is_err());
    }
}
