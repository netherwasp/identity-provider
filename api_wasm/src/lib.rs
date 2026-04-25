use base64::{Engine as _, engine::general_purpose};
use sha3::{Digest, Sha3_256};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response, Window};

use crate::api_client::ApiClient;
mod api_client;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use serde_wasm_bindgen::to_value;

const HOST_IDP: &str = "http://127.0.0.1:3000";
const HOST_AUTH: &str = "http://127.0.0.1:4000";

// REQUEST STRUCT
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

#[wasm_bindgen]
pub async fn authentication_request(json_string: &str) -> Result<JsValue, JsValue> {
    let api = ApiClient::new(HOST_IDP);
    match serde_json::from_str::<AuthLogin>(json_string) {
        Ok(mut auth_json) => {
            let response = api
                .post(
                    "/auth/login",
                    Some(
                        serde_json::to_string::<AuthLogin>(&auth_json.hash_password())
                            .unwrap()
                            .as_str(),
                    ),
                )
                .await?;

            Ok(response)
        }
        _ => Err("Request Error".into()),
    }
}
