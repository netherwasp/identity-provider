use axum::{
    Json, Router,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

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
        csrf_config: CsrfConfig::default(),
    };

    let app = Router::new()
        .fallback_service(
            ServeDir::new("src/priv/browser")
                .fallback(ServeFile::new("src/priv/browser/index.html")),
        )
        .route("/test", get(test))
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

async fn auth_csrf(token: CsrfToken) -> impl IntoResponse {
    match token.authenticity_token() {
        Ok(csrf_value) => Ok(Json(json!({ "token": csrf_value }))),
        Err(e) => Err((StatusCode::UNAUTHORIZED, format!("CSRF error: {:?}", e))),
    }
}

// async fn verify_csrf() -> impl IntoResponse{
//     todo!();
// }

async fn auth_login(
    token: CsrfToken,
    headers: HeaderMap,
    Json(request): Json<AuthLogin>,
) -> impl IntoResponse {
    tracing::debug!("token: {:?}", token.authenticity_token());
    let x_csrf_token = headers["X-CSRF-Token"].to_str().unwrap_or("");
    tracing::debug!("header x-csrf-token: {:?}", x_csrf_token);

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

async fn test() -> impl IntoResponse {
    println!("test");
    Json(json!({ "test": "test" }))
}
