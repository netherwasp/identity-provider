use gloo_net::http::{Method, RequestBuilder};
use js_sys::Date;
use serde::Deserialize;
use serde_json::Value;
use serde_wasm_bindgen::from_value;
use std::cell::RefCell;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::RequestCredentials;

use crate::shared::interfaces::{CsrfJson, CsrfState};

thread_local! {
    static CSRF: RefCell<Option<CsrfState>> = RefCell::new(None);
}

const CSRF_TTL_MS: f64 = 50.0 * 60.0 * 1000.0;

pub fn set_csrf(token: String) {
    CSRF.with(|c| {
        *c.borrow_mut() = Some(CsrfState {
            token,
            fetched_at: Date::now(),
        });
    });
}

pub fn get_csrf() -> Option<String> {
    CSRF.with(|c| {
        c.borrow().as_ref().and_then(|state| {
            let age = Date::now() - state.fetched_at;
            if age < CSRF_TTL_MS {
                Some(state.token.clone()) 
            } else {
                None
            }
        })
    })
}

pub fn clear_csrf() {
    CSRF.with(|c| *c.borrow_mut() = None);
}

#[derive(Debug, Clone, Default)]
pub struct ApiClient {
    pub request_uri: String,
    pub headers: Vec<(String, String)>,
    pub token: Option<String>,
    pub cookie: bool,
}

impl ApiClient {
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            request_uri: base_uri.into(),
            ..Default::default()
        }
    }
    pub fn with_header(
        mut self,
        header_key: impl Into<String>,
        header_value: impl Into<String>,
    ) -> Self {
        self.headers.push((header_key.into(), header_value.into()));
        self
    }
    pub async fn with_csrf(self) -> Result<Self, JsValue> {
        if let Some(_token) = get_csrf() {
            // return Ok(self.with_header("X-CSRF-Token", token));
            clear_csrf();
        }

        let api = ApiClient::new(&self.request_uri);
        match api.with_cookie().get("/csrf", None).await {
            Ok(response) => {
                let text = response.as_str();
                let json: CsrfJson = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                set_csrf(json.token.clone());
                Ok(self.with_header("X-CSRF-Token", json.token.clone()))
            }
            Err(e) => {
                clear_csrf();
                Err(e.into())
            }
        }
    }
    pub fn with_token(self, token: impl Into<String>) -> Self {
        self.with_header("Authorization", format!("Bearer {}", token.into()))
    }
    pub fn with_cookie(mut self) -> Self {
        self.cookie = true;
        self
    }

    pub fn request_builder(&self, method: Method, endpoint: &str) -> RequestBuilder {
        let req = RequestBuilder::new(&format!("{}{}", self.request_uri, endpoint)).method(method);

        // Apply all headers
        let req = self
            .headers
            .iter()
            .fold(req, |r, (key, value)| r.header(key, value));

        if self.cookie {
            req.credentials(RequestCredentials::Include)
        } else {
            req
        }
    }

    // API METHODS
    pub async fn get(
        &self,
        endpoint: &str,
        query: Option<Vec<(String, String)>>,
    ) -> Result<String, String> {
        // Build the request
        let response = {
            let req = self.request_builder(Method::GET, endpoint);

            // WITH QUERY
            let req = if let Some(query_params) = query {
                let param_ref = query_params.iter().map(|(k, v)| (k.as_str(), v.as_str()));
                req.query(param_ref)
            } else {
                req
            };
            req.build()
                .map_err(|e| e.to_string())?
                .send()
                .await
                .map_err(|e| e.to_string())?
                .text()
                .await
                .map_err(|e| e.to_string())
        };

        response
    }

    pub async fn post(&self, endpoint: &str, body: Option<&str>) -> Result<String, String> {
        //  Building Parameters
        let body_value: Value = body
            .filter(|s| !s.is_empty())
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or(Value::Null);

        let response = {
            let req = self.request_builder(Method::POST, endpoint);
            let req = req
                .json::<Value>(&serde_json::from_value(body_value).unwrap())
                .unwrap();

            req.send()
                .await
                .map_err(|e| e.to_string())?
                .text()
                .await
                .map_err(|e| e.to_string())
        };
        response
    }
}
