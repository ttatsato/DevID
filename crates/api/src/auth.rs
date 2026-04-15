use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Query, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repo;
use crate::state::AppState;

const SESSION_COOKIE: &str = "yokogushi_session";
const OAUTH_STATE_COOKIE: &str = "yokogushi_oauth_state";
const SESSION_DAYS: i64 = 30;

#[derive(Clone)]
pub struct AuthConfig {
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_url: String,
    pub frontend_url: String,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub github_id: i64,
    pub username: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
}

pub struct AuthUser(pub User);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let session_id = jar
            .get(SESSION_COOKIE)
            .ok_or(StatusCode::UNAUTHORIZED)?
            .value()
            .to_string();
        let user = repo::session::find_user(&state.db, &session_id)
            .await
            .map_err(|e| {
                tracing::warn!("session lookup failed: {e}");
                StatusCode::UNAUTHORIZED
            })?
            .ok_or(StatusCode::UNAUTHORIZED)?;
        Ok(AuthUser(user))
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/github/login", get(github_login))
        .route("/api/auth/github/callback", get(github_callback))
        .route("/api/auth/logout", post(logout))
        .route("/api/me", get(me))
}

fn oauth_client(cfg: &AuthConfig) -> BasicClient {
    BasicClient::new(
        ClientId::new(cfg.github_client_id.clone()),
        Some(ClientSecret::new(cfg.github_client_secret.clone())),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(cfg.github_redirect_url.clone()).unwrap())
}

async fn github_login(State(state): State<AppState>, jar: CookieJar) -> (CookieJar, Redirect) {
    let client = oauth_client(&state.auth);
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    let jar = jar.add(
        Cookie::build((OAUTH_STATE_COOKIE, csrf_token.secret().to_owned()))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .max_age(time::Duration::minutes(10))
            .build(),
    );

    (jar, Redirect::to(auth_url.as_str()))
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
    state: String,
}

async fn github_callback(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(q): Query<CallbackQuery>,
) -> Result<(CookieJar, Redirect), AuthError> {
    let stored_state = jar
        .get(OAUTH_STATE_COOKIE)
        .ok_or(AuthError::InvalidState)?
        .value()
        .to_string();
    if stored_state != q.state {
        return Err(AuthError::InvalidState);
    }

    let client = oauth_client(&state.auth);
    let token = client
        .exchange_code(AuthorizationCode::new(q.code))
        .request_async(async_http_client)
        .await
        .map_err(|e| AuthError::Oauth(e.to_string()))?;

    let gh = fetch_github_user(token.access_token().secret()).await?;
    let user = repo::user::upsert_from_github(
        &state.db,
        repo::user::GitHubUserData {
            id: gh.id,
            login: gh.login,
            name: gh.name,
            avatar_url: gh.avatar_url,
            email: gh.email,
        },
    )
    .await?;
    let session_id = repo::session::create(&state.db, user.id).await?;

    let jar = jar
        .remove(Cookie::build(OAUTH_STATE_COOKIE).path("/").build())
        .add(
            Cookie::build((SESSION_COOKIE, session_id))
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .max_age(time::Duration::days(SESSION_DAYS))
                .build(),
        );

    Ok((jar, Redirect::to(&state.auth.frontend_url)))
}

async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), AuthError> {
    if let Some(c) = jar.get(SESSION_COOKIE) {
        repo::session::delete(&state.db, c.value()).await?;
    }
    let jar = jar.remove(Cookie::build(SESSION_COOKIE).path("/").build());
    Ok((jar, StatusCode::NO_CONTENT))
}

async fn me(AuthUser(user): AuthUser) -> Json<User> {
    Json(user)
}

#[derive(Deserialize)]
struct GitHubUser {
    id: i64,
    login: String,
    name: Option<String>,
    avatar_url: Option<String>,
    email: Option<String>,
}

async fn fetch_github_user(token: &str) -> Result<GitHubUser, AuthError> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {token}"))
        .header("User-Agent", "yokogushi")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| AuthError::GitHubApi(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(AuthError::GitHubApi(format!("status {}", resp.status())));
    }
    resp.json::<GitHubUser>()
        .await
        .map_err(|e| AuthError::GitHubApi(e.to_string()))
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AuthError {
    InvalidState,
    Oauth(String),
    GitHubApi(String),
    Db(sqlx::Error),
}

impl From<sqlx::Error> for AuthError {
    fn from(e: sqlx::Error) -> Self {
        AuthError::Db(e)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            AuthError::InvalidState => {
                (StatusCode::BAD_REQUEST, "invalid oauth state").into_response()
            }
            other => {
                tracing::error!("auth error: {other:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "auth error").into_response()
            }
        }
    }
}
