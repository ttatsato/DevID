use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::repo;
use crate::state::AppState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Profile {
    pub display_name: Option<String>,
    pub headline: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub contact_email: Option<String>,
    #[serde(default)]
    pub contact_email_public: bool,
    #[serde(default)]
    pub social_links: Vec<SocialLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLink {
    pub platform: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/me/profile", get(get_my_profile).put(update_my_profile))
}

async fn get_my_profile(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Profile>, ProfileError> {
    let p = repo::profile::get_by_user(&state.db, user.id)
        .await?
        .unwrap_or_default();
    Ok(Json(p))
}

async fn update_my_profile(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Profile>,
) -> Result<Json<Profile>, ProfileError> {
    repo::profile::upsert(&state.db, user.id, &body).await?;
    Ok(Json(body))
}

pub enum ProfileError {
    Internal(String),
}

impl IntoResponse for ProfileError {
    fn into_response(self) -> axum::response::Response {
        let ProfileError::Internal(m) = self;
        tracing::error!("profile error: {m}");
        (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
    }
}

impl From<sqlx::Error> for ProfileError {
    fn from(e: sqlx::Error) -> Self {
        ProfileError::Internal(e.to_string())
    }
}
