use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::types::Json as SqlxJson;

use crate::auth::AuthUser;
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
    let row: Option<ProfileRow> = sqlx::query_as(
        "SELECT display_name, headline, bio, avatar_url, location, contact_email, \
                contact_email_public, social_links \
         FROM profiles WHERE user_id = $1",
    )
    .bind(user.id)
    .fetch_optional(&state.db)
    .await?;

    Ok(Json(row.map(Into::into).unwrap_or_default()))
}

async fn update_my_profile(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Profile>,
) -> Result<Json<Profile>, ProfileError> {
    let social = serde_json::to_value(&body.social_links)?;
    sqlx::query(
        "INSERT INTO profiles \
            (user_id, display_name, headline, bio, avatar_url, location, \
             contact_email, contact_email_public, social_links) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
         ON CONFLICT (user_id) DO UPDATE SET \
            display_name=EXCLUDED.display_name, headline=EXCLUDED.headline, bio=EXCLUDED.bio, \
            avatar_url=EXCLUDED.avatar_url, location=EXCLUDED.location, \
            contact_email=EXCLUDED.contact_email, contact_email_public=EXCLUDED.contact_email_public, \
            social_links=EXCLUDED.social_links, updated_at=NOW()",
    )
    .bind(user.id)
    .bind(&body.display_name)
    .bind(&body.headline)
    .bind(&body.bio)
    .bind(&body.avatar_url)
    .bind(&body.location)
    .bind(&body.contact_email)
    .bind(body.contact_email_public)
    .bind(&social)
    .execute(&state.db)
    .await?;

    Ok(Json(body))
}

#[derive(sqlx::FromRow)]
struct ProfileRow {
    display_name: Option<String>,
    headline: Option<String>,
    bio: Option<String>,
    avatar_url: Option<String>,
    location: Option<String>,
    contact_email: Option<String>,
    contact_email_public: bool,
    social_links: SqlxJson<Vec<SocialLink>>,
}

impl From<ProfileRow> for Profile {
    fn from(r: ProfileRow) -> Self {
        Self {
            display_name: r.display_name,
            headline: r.headline,
            bio: r.bio,
            avatar_url: r.avatar_url,
            location: r.location,
            contact_email: r.contact_email,
            contact_email_public: r.contact_email_public,
            social_links: r.social_links.0,
        }
    }
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
impl From<serde_json::Error> for ProfileError {
    fn from(e: serde_json::Error) -> Self {
        ProfileError::Internal(e.to_string())
    }
}
