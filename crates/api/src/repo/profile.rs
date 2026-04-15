use sqlx::types::Json as SqlxJson;
use sqlx::PgPool;
use uuid::Uuid;

use crate::profile::{Profile, SocialLink};

pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> sqlx::Result<Option<Profile>> {
    let row = sqlx::query!(
        r#"SELECT display_name, headline, bio, avatar_url, location, contact_email,
                  contact_email_public,
                  social_links as "social_links: SqlxJson<Vec<SocialLink>>"
             FROM profiles WHERE user_id = $1"#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Profile {
        display_name: r.display_name,
        headline: r.headline,
        bio: r.bio,
        avatar_url: r.avatar_url,
        location: r.location,
        contact_email: r.contact_email,
        contact_email_public: r.contact_email_public,
        social_links: r.social_links.0,
    }))
}

pub async fn upsert(pool: &PgPool, user_id: Uuid, p: &Profile) -> sqlx::Result<()> {
    let social = serde_json::to_value(&p.social_links)
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    sqlx::query!(
        "INSERT INTO profiles \
            (user_id, display_name, headline, bio, avatar_url, location, \
             contact_email, contact_email_public, social_links) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
         ON CONFLICT (user_id) DO UPDATE SET \
            display_name=EXCLUDED.display_name, headline=EXCLUDED.headline, bio=EXCLUDED.bio, \
            avatar_url=EXCLUDED.avatar_url, location=EXCLUDED.location, \
            contact_email=EXCLUDED.contact_email, contact_email_public=EXCLUDED.contact_email_public, \
            social_links=EXCLUDED.social_links, updated_at=NOW()",
        user_id,
        p.display_name,
        p.headline,
        p.bio,
        p.avatar_url,
        p.location,
        p.contact_email,
        p.contact_email_public,
        social
    )
    .execute(pool)
    .await?;
    Ok(())
}
