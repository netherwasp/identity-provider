use base64::{Engine as _, engine::general_purpose};
use sha3::{Digest, Sha3_256};
use serde::{Deserialize, Serialize};

pub struct CsrfState {
    pub token: String,
    pub fetched_at: f64, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsrfJson {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthLogin {
    pub username: String,
    pub password: String,
}

impl AuthLogin {
    pub fn hash_password(&mut self) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(self.password.clone());
        AuthLogin {
            username: self.username.clone(),
            password: general_purpose::STANDARD_NO_PAD.encode(hasher.finalize()),
        }
    }
}