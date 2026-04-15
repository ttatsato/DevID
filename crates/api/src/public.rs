use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use uuid::Uuid;
use yokogushi_core::resume::Employment;

use crate::profile::Profile;
use crate::repo;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/users/:username", get(get_public_user))
}

#[derive(Serialize)]
struct PublicUser {
    username: String,
    name: Option<String>,
    avatar_url: Option<String>,
}

#[derive(Serialize)]
struct PublicPortfolio {
    id: Uuid,
    employments: Vec<Employment>,
}

#[derive(Serialize)]
struct PublicUserResponse {
    user: PublicUser,
    profile: Profile,
    portfolio: Option<PublicPortfolio>,
}

async fn get_public_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<PublicUserResponse>, ApiError> {
    let user = repo::user::find_by_username(&state.db, &username)
        .await?
        .ok_or(ApiError::NotFound)?;

    let mut profile = repo::profile::find_by_user(&state.db, user.id)
        .await?
        .unwrap_or_default();
    if !profile.contact_email_public {
        profile.contact_email = None;
    }

    let portfolio = repo::portfolio::find_by_user(&state.db, user.id)
        .await?
        .map(|p| {
            let mut employments = p.employments;
            sanitize_for_public(&mut employments);
            PublicPortfolio {
                id: p.id,
                employments,
            }
        });

    Ok(Json(PublicUserResponse {
        user: PublicUser {
            username: user.username,
            name: user.name,
            avatar_url: user.avatar_url,
        },
        profile,
        portfolio,
    }))
}

fn sanitize_for_public(employments: &mut [Employment]) {
    for emp in employments {
        if emp.company_anonymized {
            emp.company_name = "(匿名)".into();
        }
        for proj in &mut emp.projects {
            if proj.client_anonymized {
                proj.client_name = None;
            }
        }
    }
}

enum ApiError {
    NotFound,
    Internal(String),
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError::Internal(e.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found").into_response(),
            ApiError::Internal(m) => {
                tracing::error!("public error: {m}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
            }
        }
    }
}
