CREATE TABLE users (
    id         UUID PRIMARY KEY,
    github_id  BIGINT NOT NULL UNIQUE,
    username   TEXT NOT NULL UNIQUE,
    name       TEXT,
    avatar_url TEXT,
    email      TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE sessions (
    id         TEXT PRIMARY KEY,
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_sessions_user ON sessions(user_id);
CREATE INDEX idx_sessions_expires ON sessions(expires_at);

-- Dev data cleanup to allow NOT NULL user_id
DELETE FROM portfolio_skills;
DELETE FROM portfolios;

ALTER TABLE portfolios
    ADD COLUMN user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE;

CREATE UNIQUE INDEX idx_portfolios_user ON portfolios(user_id);
