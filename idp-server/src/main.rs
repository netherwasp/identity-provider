use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::{get, post},
};
use axum_csrf::{CsrfConfig, CsrfLayer};
use time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;

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

    // let cors = CorsLayer::new()
    // .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
    // .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
    // .allow_headers([
    //     header::CONTENT_TYPE,
    //     header::AUTHORIZATION,
    //     header::ACCEPT,
    //     header::COOKIE,
    //     header::SET_COOKIE,
    // ])
    // .allow_credentials(true);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::COOKIE,
            header::SET_COOKIE,
        ]);
        // .allow_credentials(true);
    
    dotenv().expect(".env file not found");

    let mut db = IdentityDatabase {
        super_admin_url: env::var("SUPER_ADMIN_URL").expect("Missing SUPER_ADMIN_URL"),
        idp_admin_url: Some(env::var("IDP_ADMIN_URL").expect("Missing IDP_ADMIN_URL")),
    };

    let state = ServerState {
        csrf_config: CsrfConfig::default()
            .with_http_only(true)
            .with_lifetime(Duration::days(1))
            .with_cookie_len(1024)
            .with_cookie_path("/"),
        database: db
            .idp_db_init()
            .await
            .unwrap()
            .idp_admin_connect()
            .await
            .unwrap(),
    };

    let session_store = PostgresStore::new(state.database.pool.clone());
    let _ = session_store.migrate().await;

    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    let auth_service = ServiceBuilder::new().service(
        ServeDir::new("src/priv/auth_service/browser")
            .fallback(ServeFile::new("src/priv/auth_service/browser/index.html")),
    );

    tracing::debug!("state {:?}", state);
    let app = Router::new()
        .nest_service("/auth", auth_service)
        .route("/auth/login", post(auth_login_handler))
        .route("/csrf", get(csrf_handler))
        .with_state(state.clone())
        .layer(CsrfLayer::new(state.csrf_config.clone()))
        .layer(session_layer)
        // layer for database
        .layer(cors);

    let addr = "127.0.0.1:3000";
    tracing::debug!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// Middlewares
