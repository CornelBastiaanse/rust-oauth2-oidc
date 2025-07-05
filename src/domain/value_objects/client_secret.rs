use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::domain::traits::value_object::ValueObject;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientSecret {
    hash: String,
}

impl ClientSecret {
    /// Creates a new hashed secret from the raw password using Argon2 and a provided salt string.
    pub fn new(raw: &str, salt: &SaltString) -> Result<Self, String> {
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(raw.as_bytes(), salt)
            .map_err(|e| e.to_string())?
            .to_string();

        Ok(Self { hash })
    }

    /// Instantiates a `ClientSecret` from a previously hashed string (e.g., from a database).
    pub fn from_hash(hash: String) -> Self {
        Self { hash }
    }

    /// Verifies a raw input against the stored hash.
    pub fn verify(&self, attempt: &str) -> bool {
        let argon2 = Argon2::default();

        PasswordHash::new(&self.hash)
            .and_then(|parsed| argon2.verify_password(attempt.as_bytes(), &parsed))
            .is_ok()
    }

    /// Returns the hash as a string reference.
    pub fn hash(&self) -> &str {
        &self.hash
    }
}

impl ValueObject for ClientSecret {}