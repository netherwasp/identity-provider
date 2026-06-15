use wasm_bindgen::prelude::*;
mod api_client;
mod shared;
use api_client::{ApiClient, clear_csrf, get_csrf, set_csrf};

use shared::interfaces::{AuthLogin, CsrfJson};

const HOST_IDP: &str = "http://127.0.0.1:3000";
const HOST_AUTH: &str = "http://127.0.0.1:4000";

#[wasm_bindgen]
pub async fn ensure_csrf() -> Result<String, String> {
    if let Some(token) = get_csrf() {
        return Ok(token);
    }

    let api = ApiClient::new(HOST_IDP);
    match api.with_cookie().get_csrf().await {
        Ok((_, Some(token))) => {
            set_csrf(token.clone());
            Ok(token)
        }
        Ok((_, None)) => Err("No X-CSRF-Token in response header".into()),
        Err(e) => {
            clear_csrf();
            Err(e)
        }
    }
}

#[wasm_bindgen]
pub async fn authentication_request(json_string: &str) -> Result<JsValue, JsValue> {
    let api = ApiClient::new(HOST_IDP);
    // let csrf = ensure_csrf().await?;

    match serde_json::from_str::<AuthLogin>(json_string) {
        Ok(mut auth_json) => {
            let response = api
                .with_cookie()
                .with_csrf()
                .await?
                .post(
                    "/auth/login",
                    Some(
                        serde_json::to_string::<AuthLogin>(&auth_json.hash_password())
                            .unwrap()
                            .as_str(),
                    ),
                )
                .await?;

            Ok(response.into())
        }
        _ => Err(format!("Failed to parse AuthLogin from: {}", json_string).into()),
    }
}
