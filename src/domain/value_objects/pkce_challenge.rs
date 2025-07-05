use sha2::{Digest, Sha256};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use crate::domain::enums::pkce_method::PKCEMethod;
use crate::domain::traits::value_object::ValueObject;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PKCEChallenge {
    method: PKCEMethod,
    challenge: String
}

impl PKCEChallenge {
    pub fn new(method: PKCEMethod, challenge: String) -> Self {
        Self { method, challenge }
    }

    pub fn verify(&self, verifier: &str) -> bool {
        match self.method {
            PKCEMethod::Plain => self.challenge == verifier,
            PKCEMethod::S256 => {
                let hash = Sha256::digest(verifier.as_bytes());
                let encoded = URL_SAFE_NO_PAD.encode(hash);
                self.challenge == encoded
            }
        }
    }

    pub fn method(&self) -> &PKCEMethod {
        &self.method
    }

    pub fn challenge(&self) -> &str {
        &self.challenge
    }
}

impl ValueObject for PKCEChallenge {}