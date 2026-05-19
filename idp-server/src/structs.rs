use axum_csrf::CsrfConfig;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// Server State
#[derive(Debug, Clone)]
pub struct ServerState {
    pub csrf_config: CsrfConfig,
    pub database_state: PgPool,
}

// REQUEST STRUCT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthLogin {
    pub username: String,
    pub password: String,
}
