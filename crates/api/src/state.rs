use sqlx::PgPool;
use std::sync::Arc;

use crate::auth::AuthConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub auth: Arc<AuthConfig>,
}
