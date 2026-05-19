use axum::{
    Router,
    routing::{get, post},
};
use axum_csrf::{CsrfConfig, CsrfLayer};
use time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_sessions::{MemoryStore, SessionManagerLayer};

use dotenvy::dotenv;
use std::env;

mod utils;

mod database;

mod server;
use server::handlers::{auth_login_handler, csrf_handler};

mod structs;
use structs::ServerState;

use crate::database::IdentityDatabase;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    dotenv().expect(".env file not found");

    let mut db = IdentityDatabase {
        super_admin_url: env::var("SUPER_ADMIN_URL").unwrap_or_default(),
        idp_admin_url: Some(env::var("IDP_ADMIN_URL").unwrap_or_default()),
    };

    let state = ServerState {
        csrf_config: CsrfConfig::default()
            .with_http_only(true)
            .with_lifetime(Duration::days(1))
            .with_cookie_len(1024)
            .with_cookie_path("/"),
        database_state: db
            .idp_db_init()
            .await
            .unwrap()
            .idp_admin_connect()
            .await
            .unwrap(),
    };

    let session = SessionManagerLayer::new(MemoryStore::default()).with_secure(false);

    let login_service = ServiceBuilder::new().service(
        ServeDir::new("src/priv/browser").fallback(ServeFile::new("src/priv/browser/index.html")),
    );

    let app = Router::new()
        .nest_service("/auth", login_service)
        .route("/auth/login", post(auth_login_handler))
        .route("/csrf", get(csrf_handler))
        .with_state(state.clone())
        .layer(CsrfLayer::new(state.csrf_config.clone()))
        .layer(session)
        // layer for database
        .layer(cors);

    let addr = "127.0.0.1:3000";
    tracing::debug!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// Middlewares
