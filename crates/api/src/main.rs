mod auth;
mod db;
mod portfolio;
mod profile;
mod public;
mod repo;
mod state;

use axum::{extract::Query, routing::get, Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use yokogushi_core::{Certification, Skill};
use yokogushi_dict::{suggest_certifications, suggest_skills};

use auth::AuthConfig;
use state::AppState;

#[derive(Deserialize)]
struct SuggestParams {
    q: String,
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    10
}

async fn skills_suggest(Query(p): Query<SuggestParams>) -> Json<Vec<&'static Skill>> {
    Json(suggest_skills(&p.q, p.limit))
}

async fn certs_suggest(Query(p): Query<SuggestParams>) -> Json<Vec<&'static Certification>> {
    Json(suggest_certifications(&p.q, p.limit))
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "yokogushi_api=info,tower_http=info,sqlx=warn".into()),
        )
        .init();

    let database_url = env_or(
        "DATABASE_URL",
        "postgres://yokogushi:yokogushi@localhost:5434/yokogushi",
    );
    let auth = AuthConfig {
        github_client_id: std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set"),
        github_client_secret: std::env::var("GITHUB_CLIENT_SECRET")
            .expect("GITHUB_CLIENT_SECRET must be set"),
        github_redirect_url: env_or(
            "GITHUB_REDIRECT_URL",
            "http://localhost:3001/api/auth/github/callback",
        ),
        frontend_url: env_or("FRONTEND_URL", "http://localhost:3000"),
    };

    tracing::info!("connecting to database");
    let pool = db::connect(&database_url).await?;
    db::migrate(&pool).await?;
    tracing::info!("migrations applied");

    let app_state = AppState {
        db: pool,
        auth: Arc::new(auth),
    };

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/dict/skills", get(skills_suggest))
        .route("/api/dict/certs", get(certs_suggest))
        .merge(auth::routes())
        .merge(profile::routes())
        .merge(portfolio::routes())
        .merge(public::routes())
        .with_state(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
