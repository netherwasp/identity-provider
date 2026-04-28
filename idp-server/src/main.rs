use axum::{
    Json, Router,
    http::{HeaderMap, StatusCode, header::SET_COOKIE},
    response::IntoResponse,
    routing::{get, post},
};
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken};
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use uuid::Uuid;

mod utils;
use crate::utils::{generate_nonce, parse_cookie, set_cookie};
// REQUEST STRUCT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthLogin {
    username: String,
    password: String,
}

// Server State
#[derive(Clone)]
struct AppState {
    csrf_config: CsrfConfig,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let state = AppState {
        csrf_config: CsrfConfig::default()
            .with_http_only(true)
            .with_lifetime(Duration::days(1))
            .with_cookie_len(1024)
            .with_cookie_path("/csrf"),
    };

    let app = Router::new()
        .fallback_service(
            ServeDir::new("src/priv/browser")
                .fallback(ServeFile::new("src/priv/browser/index.html")),
        )
        .route("/csrf", get(auth_csrf))
        .route("/auth/login", post(auth_login))
        .with_state(state.clone())
        .layer(CsrfLayer::new(state.csrf_config.clone()))
        .layer(cors);

    let addr = "127.0.0.1:3000";
    tracing::debug!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn auth_csrf(token: CsrfToken, headers: HeaderMap) -> impl IntoResponse {
    let cookie = headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    let device_id = parse_cookie(cookie, "device_id").unwrap_or_else(|| Uuid::new_v4().to_string());

    let nonce = parse_cookie(cookie, "nonce").unwrap_or_else(|| generate_nonce(12));

    match token.authenticity_token() {
        Ok(csrf_value) => {
            let mut response = Json(json!({ "token": csrf_value })).into_response();
            // adding specific details needed
            set_cookie(&mut response, "device_id", &device_id, "/csrf", 31536000);
            set_cookie(&mut response, "nonce", &nonce, "/csrf", 3600);

            (token, response).into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, format!("CSRF error: {:?}", e)).into_response(),
    }
}

// async fn verify_csrf() -> impl IntoResponse{
//     todo!();
// }

async fn auth_login(
    // token: CsrfToken,
    // headers: HeaderMap,
    Json(request): Json<AuthLogin>,
) -> impl IntoResponse {
    // tracing::debug!("token: {:?}", token.authenticity_token());
    // let x_csrf_token = headers
    //     .get("X-CSRF-Token")
    //     .and_then(|v| v.to_str().ok())
    //     .unwrap_or("");
    // tracing::debug!("header x-csrf-token: {:?}", x_csrf_token);

    // if token.verify(x_csrf_token).is_err() {
    //     return (
    //         StatusCode::FORBIDDEN,
    //         Json(json!({ "error": "Invalid CSRF token" })),
    //     )
    //         .into_response();
    // }

    println!("request: {:?}", request);
    (StatusCode::OK, Json(json!({ "test": "test" }))).into_response()
}
