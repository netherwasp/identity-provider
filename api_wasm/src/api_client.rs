use gloo_net::http::{Method, RequestBuilder};
use js_sys::{Array, Reflect};
use serde::Deserialize;
use serde_json::Value;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use web_sys::RequestCredentials;
#[derive(Debug, Clone)]
pub struct ApiClient {
    pub request_uri: String,
    pub header: Option<(String, String)>,
    pub token: Option<String>,
    pub cookie: bool,
}

impl ApiClient {
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            request_uri: base_uri.into(),
            header: None,
            token: None,
            cookie: false,
        }
    }
    pub fn with_header(
        mut self,
        header_key: impl Into<String>,
        header_value: impl Into<String>,
    ) -> Self {
        self.header = Some((header_key.into(), header_value.into()));
        self
    }
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }
    pub fn with_cookie(mut self) -> Self {
        self.cookie = true;
        self
    }
    // API METHODS
    pub async fn get(&self, endpoint: &str, query: Option<JsValue>) -> Result<JsValue, JsValue> {
        // Build the request
        let request = {
            let req = RequestBuilder::new(&format!("{}{}", self.request_uri, endpoint))
                .method(Method::GET);
            // WITH TOKEN .header("Authorization", "Bearer {token}")
            let req = if let Some((header_key, header_value)) = &self.header {
                req.header(header_key, header_value)
            } else {
                req
            };

            // WITH COOKIE
            let req = if self.cookie {
                req.credentials(RequestCredentials::Include)
            } else {
                req
            };

            // WITH QUERY
            let get_req = if let Some(query_params) = query {
                if let Ok(params) =
                    serde_wasm_bindgen::from_value::<Vec<(String, String)>>(query_params)
                {
                    let param_ref = params.iter().map(|(k, v)| (k.as_str(), v.as_str()));
                    req.query(param_ref)
                } else {
                    req
                }
            } else {
                req
            };
            get_req.build().unwrap()
        };

        let response = request
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .text()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(JsValue::from_str(&response))
    }

    pub async fn post(&self, endpoint: &str, body: Option<&str>) -> Result<JsValue, JsValue> {
        //  Building Parameters
        let body_json = body.unwrap_or("");
        let request = {
            let req = RequestBuilder::new(&format!("{}{}", self.request_uri, endpoint))
                .method(Method::POST);
            // WITH TOKEN .header("Authorization", "Bearer {token}")
            let req = if let Some((header_key, header_value)) = &self.header {
                req.header(header_key, header_value)
            } else {
                req
            };

            // WITH COOKIE
            let req = if self.cookie {
                req.credentials(RequestCredentials::Include)
            } else {
                req
            };

            let post_req = req
                .json::<Value>(&serde_json::from_str(body_json).unwrap())
                .unwrap();
            post_req
        };

        let response = request
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .text()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(JsValue::from_str(&response))
    }
}
