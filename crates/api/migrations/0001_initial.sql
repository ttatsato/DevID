CREATE TABLE portfolios (
    id         UUID PRIMARY KEY,
    data       JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE portfolio_skills (
    portfolio_id   UUID NOT NULL REFERENCES portfolios(id) ON DELETE CASCADE,
    skill_id       TEXT NOT NULL,
    total_months   INTEGER NOT NULL,
    primary_months INTEGER NOT NULL,
    last_used      DATE NOT NULL,
    project_count  INTEGER NOT NULL,
    PRIMARY KEY (portfolio_id, skill_id)
);

CREATE INDEX idx_portfolio_skills_skill ON portfolio_skills(skill_id);
CREATE INDEX idx_portfolio_skills_lookup
    ON portfolio_skills(skill_id, total_months DESC);
