mod db;
mod portfolio;

use axum::{extract::Query, routing::get, Json, Router};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use yokogushi_core::{Certification, Skill};
use yokogushi_dict::{suggest_certifications, suggest_skills};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "yokogushi_api=info,tower_http=info,sqlx=warn".into()),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://yokogushi:yokogushi@localhost:5434/yokogushi".to_string()
    });

    tracing::info!("connecting to database");
    let pool = db::connect(&database_url).await?;
    db::migrate(&pool).await?;
    tracing::info!("migrations applied");

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/dict/skills", get(skills_suggest))
        .route("/api/dict/certs", get(certs_suggest))
        .merge(portfolio::routes(pool))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
