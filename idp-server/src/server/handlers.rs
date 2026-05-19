use axum::{
    Json, extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse
};

use serde_json::json;

use axum_csrf::CsrfToken;
use tower_sessions::Session;

use crate::structs::{AuthLogin, ServerState};

// API handlers
pub async fn csrf_handler(session: Session, token: CsrfToken) -> impl IntoResponse {
    println!("session: {:?}", session);

    let mut response_header = HeaderMap::new();
    match token.authenticity_token() {
        Ok(csrf_value) => {
            response_header.insert("X-CSRF-Token", csrf_value.parse().unwrap());
            Ok((response_header, StatusCode::NO_CONTENT))
        }
        Err(e) => Err((StatusCode::UNAUTHORIZED, format!("CSRF error: {:?}", e))),
    }
}

pub async fn auth_login_handler(
    State(state): State<ServerState>,
    session: Session,
    Json(request): Json<AuthLogin>,
) -> impl IntoResponse {
    println!("request: {:?}", request);
    println!("state: {:?}", state);
    println!("session: {:?}", session);
    (StatusCode::OK, Json(json!({ "test": "test" }))).into_response()
}
