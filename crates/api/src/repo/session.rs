use base64::Engine;
use chrono::{Duration, Utc};
use rand::RngCore;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::User;

const SESSION_DAYS: i64 = 30;

pub async fn create(pool: &PgPool, user_id: Uuid) -> sqlx::Result<String> {
    let session_id = random_token(32);
    let expires_at = Utc::now() + Duration::days(SESSION_DAYS);
    sqlx::query!(
        "INSERT INTO sessions (id, user_id, expires_at) VALUES ($1, $2, $3)",
        session_id,
        user_id,
        expires_at
    )
    .execute(pool)
    .await?;
    Ok(session_id)
}

pub async fn delete(pool: &PgPool, session_id: &str) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM sessions WHERE id = $1", session_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// セッションIDから現在のユーザーを解決。期限切れは None。
pub async fn find_user_by_session_id(
    pool: &PgPool,
    session_id: &str,
) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        "SELECT u.id, u.github_id, u.username, u.name, u.avatar_url, u.email \
         FROM sessions s JOIN users u ON u.id = s.user_id \
         WHERE s.id = $1 AND s.expires_at > NOW()",
        session_id
    )
    .fetch_optional(pool)
    .await
}

fn random_token(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    rand::thread_rng().fill_bytes(&mut buf);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(buf)
}
