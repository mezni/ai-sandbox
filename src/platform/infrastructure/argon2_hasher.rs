//! Argon2id password hashing adapter.

use argon2::password_hash::phc::PasswordHash;
use argon2::password_hash::{PasswordHasher as _, PasswordVerifier};
use argon2::{Algorithm, Argon2, Params, Version};

use crate::platform::application::ports::PasswordHasher;
use crate::platform::domain::errors::PlatformError;

#[derive(Debug, Clone)]
pub struct Argon2idHasher {
    memory: u32,
    time_cost: u32,
    threads: u32,
}

impl Argon2idHasher {
    pub fn new(memory: u32, time_cost: u32, threads: u32) -> Self {
        Self {
            memory,
            time_cost,
            threads,
        }
    }

    fn argon2(&self) -> Result<Argon2<'static>, PlatformError> {
        let params = Params::new(self.memory, self.time_cost, self.threads, None)
            .map_err(|err| PlatformError::Internal(format!("invalid Argon2 parameters: {err}")))?;
        Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
    }
}

impl PasswordHasher for Argon2idHasher {
    fn hash(&self, password: &str) -> Result<String, PlatformError> {
        let hash = self
            .argon2()?
            .hash_password(password.as_bytes())
            .map_err(|err| PlatformError::Internal(format!("password hashing failed: {err}")))?;
        Ok(hash.to_string())
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool, PlatformError> {
        let parsed = PasswordHash::new(hash)
            .map_err(|err| PlatformError::Internal(format!("invalid password hash: {err}")))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_and_verifies_passwords() {
        let hasher = Argon2idHasher::new(19_456, 2, 1);
        let hash = hasher.hash("S3cure-passphrase!").unwrap();
        assert!(hash.starts_with("$argon2id$"));
        assert!(hasher.verify("S3cure-passphrase!", &hash).unwrap());
        assert!(!hasher.verify("wrong-password", &hash).unwrap());
    }
}
