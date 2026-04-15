CREATE TABLE profiles (
    user_id              UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    display_name         TEXT,
    headline             TEXT,
    bio                  TEXT,
    avatar_url           TEXT,
    location             TEXT,
    contact_email        TEXT,
    contact_email_public BOOLEAN NOT NULL DEFAULT FALSE,
    social_links         JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 既存ユーザーに空プロフィールを作成（GitHub 由来の name / avatar をデフォルトに）
INSERT INTO profiles (user_id, display_name, avatar_url)
SELECT id, name, avatar_url FROM users
ON CONFLICT DO NOTHING;
