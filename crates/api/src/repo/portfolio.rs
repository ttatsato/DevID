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
    let row = sqlx::query!(
        r#"SELECT id, data as "data: SqlxJson<Vec<Employment>>"
             FROM portfolios WHERE user_id = $1"#,
        user_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| PortfolioRecord {
        id: r.id,
        employments: r.data.0,
    }))
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<PortfolioRecord>> {
    let row = sqlx::query!(
        r#"SELECT data as "data: SqlxJson<Vec<Employment>>"
             FROM portfolios WHERE id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| PortfolioRecord {
        id,
        employments: r.data.0,
    }))
}

pub async fn upsert_for_user(
    pool: &PgPool,
    user_id: Uuid,
    employments: &[Employment],
    experience: &[SkillExperience],
) -> sqlx::Result<Uuid> {
    let data = serde_json::to_value(employments).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    let new_id = Uuid::new_v4();
    let mut tx = pool.begin().await?;

    let id = sqlx::query_scalar!(
        "INSERT INTO portfolios (id, user_id, data) VALUES ($1, $2, $3) \
         ON CONFLICT (user_id) DO UPDATE SET data = EXCLUDED.data, updated_at = NOW() \
         RETURNING id",
        new_id,
        user_id,
        data
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query!("DELETE FROM portfolio_skills WHERE portfolio_id = $1", id)
        .execute(&mut *tx)
        .await?;

    for e in experience {
        sqlx::query!(
            "INSERT INTO portfolio_skills \
             (portfolio_id, skill_id, total_months, primary_months, last_used, project_count) \
             VALUES ($1, $2, $3, $4, $5, $6)",
            id,
            e.skill_id,
            e.total_months as i32,
            e.primary_months as i32,
            e.last_used,
            e.project_count as i32
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(id)
}

pub async fn exists_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
    let row = sqlx::query_scalar!("SELECT id FROM portfolios WHERE id = $1", id)
        .fetch_optional(pool)
        .await?;
    Ok(row.is_some())
}

pub async fn list_skill_experience(
    pool: &PgPool,
    portfolio_id: Uuid,
) -> sqlx::Result<Vec<SkillExperience>> {
    let rows = sqlx::query!(
        "SELECT skill_id, total_months, primary_months, last_used, project_count \
         FROM portfolio_skills \
         WHERE portfolio_id = $1 \
         ORDER BY total_months DESC, skill_id ASC",
        portfolio_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| SkillExperience {
            skill_id: r.skill_id,
            total_months: r.total_months as u32,
            primary_months: r.primary_months as u32,
            last_used: r.last_used,
            project_count: r.project_count as u32,
        })
        .collect())
}
