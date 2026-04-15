use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use yokogushi_core::resume::Employment;
use yokogushi_core::skill_aggregation::{aggregate_skill_experience, SkillExperience};

#[derive(Default)]
pub struct AppState {
    portfolios: RwLock<HashMap<Uuid, Vec<Employment>>>,
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/portfolios", post(create_portfolio))
        .route("/api/portfolios/:id", get(get_portfolio))
        .route(
            "/api/portfolios/:id/skill-experience",
            get(get_skill_experience),
        )
        .with_state(state)
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
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreatePortfolioRequest>,
) -> Json<CreatePortfolioResponse> {
    let id = Uuid::new_v4();
    state.portfolios.write().await.insert(id, body.employments);
    Json(CreatePortfolioResponse { id })
}

#[derive(Serialize)]
pub struct PortfolioResponse {
    pub id: Uuid,
    pub employments: Vec<Employment>,
}

async fn get_portfolio(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<PortfolioResponse>, StatusCode> {
    let store = state.portfolios.read().await;
    let employments = store.get(&id).ok_or(StatusCode::NOT_FOUND)?.clone();
    Ok(Json(PortfolioResponse { id, employments }))
}

async fn get_skill_experience(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<SkillExperience>>, StatusCode> {
    let store = state.portfolios.read().await;
    let employments = store.get(&id).ok_or(StatusCode::NOT_FOUND)?;
    let today = Utc::now().date_naive();
    Ok(Json(aggregate_skill_experience(employments, today)))
}

impl IntoResponse for CreatePortfolioResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
