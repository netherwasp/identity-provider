use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

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
    pub fn hash_password(&mut self) -> &mut Self {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.password);

        self.password = general_purpose::STANDARD_NO_PAD.encode(hasher.finalize());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRegister {
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub phone_number: String,
    pub username: String,
    pub password: String,
}

impl AuthRegister {
    pub fn hash_password(&mut self) -> &mut Self {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.password);

        self.password = general_purpose::STANDARD_NO_PAD.encode(hasher.finalize());
        self
    }
}
