use chrono::NaiveDate;
use sqlx::types::Json as SqlxJson;
use sqlx::PgPool;
use uuid::Uuid;
use yokogushi_core::resume::Employment;
use yokogushi_core::skill_aggregation::SkillExperience;

pub struct PortfolioRecord {
    pub id: Uuid,
    pub employments: Vec<Employment>,
}

pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> sqlx::Result<Option<PortfolioRecord>> {
    let row: Option<(Uuid, SqlxJson<Vec<Employment>>)> =
        sqlx::query_as("SELECT id, data FROM portfolios WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
    Ok(row.map(|(id, data)| PortfolioRecord {
        id,
        employments: data.0,
    }))
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<PortfolioRecord>> {
    let row: Option<(SqlxJson<Vec<Employment>>,)> =
        sqlx::query_as("SELECT data FROM portfolios WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    Ok(row.map(|(data,)| PortfolioRecord {
        id,
        employments: data.0,
    }))
}

pub async fn upsert_for_user(
    pool: &PgPool,
    user_id: Uuid,
    employments: &[Employment],
    experience: &[SkillExperience],
) -> sqlx::Result<Uuid> {
    let data = serde_json::to_value(employments).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    let mut tx = pool.begin().await?;

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO portfolios (id, user_id, data) VALUES ($1, $2, $3) \
         ON CONFLICT (user_id) DO UPDATE SET data = EXCLUDED.data, updated_at = NOW() \
         RETURNING id",
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&data)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query("DELETE FROM portfolio_skills WHERE portfolio_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    for e in experience {
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
    Ok(id)
}

pub async fn exists_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
    let row: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM portfolios WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row.is_some())
}

pub async fn list_skill_experience(
    pool: &PgPool,
    portfolio_id: Uuid,
) -> sqlx::Result<Vec<SkillExperience>> {
    let rows: Vec<(String, i32, i32, NaiveDate, i32)> = sqlx::query_as(
        "SELECT skill_id, total_months, primary_months, last_used, project_count \
         FROM portfolio_skills \
         WHERE portfolio_id = $1 \
         ORDER BY total_months DESC, skill_id ASC",
    )
    .bind(portfolio_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(skill_id, total, primary, last_used, count)| SkillExperience {
            skill_id,
            total_months: total as u32,
            primary_months: primary as u32,
            last_used,
            project_count: count as u32,
        })
        .collect())
}
