//! Authentication policy values (independent of framework configuration).

#[derive(Debug, Clone, Copy)]
pub struct AuthPolicy {
    pub access_ttl: i64,
    pub refresh_ttl: i64,
    pub remember_me_ttl: i64,
    pub password_min_length: usize,
}
