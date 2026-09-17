use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWKS {
    pub keys: Vec<JWK>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JWK {
    RSA(RSAJWK),
    EC(ECJWK),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RSAJWK {
    pub kty: String,
    pub r#use: String,
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ECJWK {
    pub kty: String,
    pub r#use: String,
    pub kid: String,
    pub alg: String,
    pub crv: String,
    pub x: String,
    pub y: String,
}
