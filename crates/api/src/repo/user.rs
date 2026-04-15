use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::User;

pub struct GitHubUserData {
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
}

/// 新規ならユーザー + 空プロフィールを作成、既存なら GitHub の最新情報で更新する。
pub async fn upsert_from_github(pool: &PgPool, gh: GitHubUserData) -> sqlx::Result<User> {
    let existing = sqlx::query_as!(
        User,
        "SELECT id, github_id, username, name, avatar_url, email \
         FROM users WHERE github_id = $1",
        gh.id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(mut u) = existing {
        sqlx::query!(
            "UPDATE users SET username=$2, name=$3, avatar_url=$4, email=$5, updated_at=NOW() \
             WHERE id=$1",
            u.id,
            gh.login,
            gh.name,
            gh.avatar_url,
            gh.email
        )
        .execute(pool)
        .await?;
        u.username = gh.login;
        u.name = gh.name;
        u.avatar_url = gh.avatar_url;
        u.email = gh.email;
        return Ok(u);
    }

    let id = Uuid::new_v4();
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "INSERT INTO users (id, github_id, username, name, avatar_url, email) \
         VALUES ($1, $2, $3, $4, $5, $6)",
        id,
        gh.id,
        gh.login,
        gh.name,
        gh.avatar_url,
        gh.email
    )
    .execute(&mut *tx)
    .await?;
    sqlx::query!(
        "INSERT INTO profiles (user_id, display_name, avatar_url) \
         VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        id,
        gh.name,
        gh.avatar_url
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    Ok(User {
        id,
        github_id: gh.id,
        username: gh.login,
        name: gh.name,
        avatar_url: gh.avatar_url,
        email: gh.email,
    })
}
