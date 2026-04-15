use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use yokogushi_core::resume::Employment;
use yokogushi_core::skill_aggregation::{aggregate_skill_experience, SkillExperience};

use crate::auth::AuthUser;
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
    let data = serde_json::to_value(&body.employments)?;

    let mut tx = state.db.begin().await?;

    // Upsert portfolio for this user (1 user = 1 portfolio)
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO portfolios (id, user_id, data) VALUES ($1, $2, $3) \
         ON CONFLICT (user_id) DO UPDATE SET data = EXCLUDED.data, updated_at = NOW() \
         RETURNING id",
    )
    .bind(Uuid::new_v4())
    .bind(user.id)
    .bind(&data)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query("DELETE FROM portfolio_skills WHERE portfolio_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    for e in &experience {
        sqlx::query(
            "INSERT INTO portfolio_skills \
             (portfolio_id, skill_id, total_months, primary_months, last_used, project_count) \
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(id)
        .bind(&e.skill_id)
        .bind(e.total_months as i32)
        .bind(e.primary_months as i32)
        .bind(e.last_used)
        .bind(e.project_count as i32)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
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
    let row: Option<(Uuid, serde_json::Value)> =
        sqlx::query_as("SELECT id, data FROM portfolios WHERE user_id = $1")
            .bind(user.id)
            .fetch_optional(&state.db)
            .await?;
    let result = row
        .map(|(id, data)| {
            let employments: Vec<Employment> = serde_json::from_value(data)?;
            Ok::<_, serde_json::Error>(PortfolioResponse { id, employments })
        })
        .transpose()?;
    Ok(Json(result))
}

async fn get_portfolio(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PortfolioResponse>, ApiError> {
    let row: Option<(serde_json::Value,)> =
        sqlx::query_as("SELECT data FROM portfolios WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db)
            .await?;
    let data = row.ok_or(ApiError::NotFound)?.0;
    let employments: Vec<Employment> = serde_json::from_value(data)?;
    Ok(Json(PortfolioResponse { id, employments }))
}

async fn get_skill_experience(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<SkillExperience>>, ApiError> {
    let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM portfolios WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    if exists.is_none() {
        return Err(ApiError::NotFound);
    }

    let rows: Vec<(String, i32, i32, NaiveDate, i32)> = sqlx::query_as(
        "SELECT skill_id, total_months, primary_months, last_used, project_count \
         FROM portfolio_skills \
         WHERE portfolio_id = $1 \
         ORDER BY total_months DESC, skill_id ASC",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    let result = rows
        .into_iter()
        .map(
            |(skill_id, total, primary, last_used, count)| SkillExperience {
                skill_id,
                total_months: total as u32,
                primary_months: primary as u32,
                last_used,
                project_count: count as u32,
            },
        )
        .collect();

    Ok(Json(result))
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

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError::Internal(e.to_string())
    }
}

// Silence unused import warning when PgPool is not used directly.
#[allow(dead_code)]
fn _unused(_: &PgPool) {}
