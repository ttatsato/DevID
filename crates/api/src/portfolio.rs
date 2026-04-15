use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use yokogushi_core::resume::Employment;
use yokogushi_core::skill_aggregation::{aggregate_skill_experience, SkillExperience};

use crate::auth::AuthUser;
use crate::repo;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/portfolios", post(create_portfolio))
        .route("/api/me/portfolio", get(get_my_portfolio))
        .route("/api/portfolios/:id", get(get_portfolio))
        .route(
            "/api/portfolios/:id/skill-experience",
            get(get_skill_experience),
        )
}

#[derive(Deserialize)]
pub struct CreatePortfolioRequest {
    pub employments: Vec<Employment>,
}

#[derive(Serialize)]
pub struct CreatePortfolioResponse {
    pub id: Uuid,
}

async fn create_portfolio(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(body): Json<CreatePortfolioRequest>,
) -> Result<Json<CreatePortfolioResponse>, ApiError> {
    let today = Utc::now().date_naive();
    let experience = aggregate_skill_experience(&body.employments, today);
    let id = repo::portfolio::upsert_for_user(&state.db, user.id, &body.employments, &experience)
        .await?;
    Ok(Json(CreatePortfolioResponse { id }))
}

#[derive(Serialize)]
pub struct PortfolioResponse {
    pub id: Uuid,
    pub employments: Vec<Employment>,
}

async fn get_my_portfolio(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Option<PortfolioResponse>>, ApiError> {
    let record = repo::portfolio::get_by_user(&state.db, user.id).await?;
    Ok(Json(record.map(|r| PortfolioResponse {
        id: r.id,
        employments: r.employments,
    })))
}

async fn get_portfolio(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PortfolioResponse>, ApiError> {
    let record = repo::portfolio::get_public(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(PortfolioResponse {
        id: record.id,
        employments: record.employments,
    }))
}

async fn get_skill_experience(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<SkillExperience>>, ApiError> {
    if !repo::portfolio::exists(&state.db, id).await? {
        return Err(ApiError::NotFound);
    }
    let items = repo::portfolio::get_skill_experience(&state.db, id).await?;
    Ok(Json(items))
}

pub enum ApiError {
    NotFound,
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found").into_response(),
            ApiError::Internal(m) => {
                tracing::error!("api error: {m}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
            }
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError::Internal(e.to_string())
    }
}
