use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
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

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new()
        .fallback_service(
            ServeDir::new("src/priv/browser").fallback(
                ServeFile::new("src/priv/browser/index.html")
            )
        )
        .route("/test", get(test))
        .route("/auth/login", post(auth_login))
        .layer(cors);

    let addr = "127.0.0.1:3000";
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn auth_login(Json(request): Json<AuthLogin>) -> impl IntoResponse {
    println!("request: {:?}", request);
    Json(json!({ "test": "test" }))
}

async fn test() -> impl IntoResponse {
    println!("test");
    Json(json!({ "test": "test" }))
}
