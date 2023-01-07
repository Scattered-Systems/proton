/*
   Appellation: auth <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use async_session::{MemoryStore, Session, SessionStore};
use axum::extract::{Extension, Path, Query, TypedHeader};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{
    headers::Cookie,
    http::{header::SET_COOKIE, HeaderMap},
    routing::{get, post},
    Json, Router,
};
use oauth2::{basic::BasicClient, reqwest::async_http_client};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use proton_sdk::auth::User;

static COOKIE_NAME: &str = "SESSION";

pub fn router(ctx: crate::Context) -> Router {
    let oauth_client = oauth_client(Extension(ctx));
    let store = MemoryStore::new();

    Router::new()
        // .route("/auth", get(index))
        .route("/token/:id", post(token))
        .route("/jetbrains", get(auth_jbspace))
        .route("/login", get(login_authorized))
        .route("/logout", get(logout))
        .layer(Extension(store))
        .layer(Extension(oauth_client))
}

// Session is optional
pub async fn index(user: Option<User>) -> impl IntoResponse {
    let msg = match user {
        Some(u) => format!(
            "Hey {}! You're logged in!\nYou may now access `/protected`.\nLog out with `/logout`.",
            u.name
        ),
        None => "You're not logged in.\nVisit `/auth/google` to do so.".to_string(),
    };
    scsys::prelude::Message::new(vec![msg]).to_string()
}

/// Implements the authorization url following the OAuth2 specification
pub async fn authorize(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(data)
}

/// Implements the OAuth2 token
pub async fn token(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(data)
}

fn oauth_client(Extension(ctx): Extension<crate::Context>) -> BasicClient {
    let client_id = ctx.cnf.auth.id;
    let client_secret = ctx.cnf.auth.secret;
    let redirect_url = "http://localhost:8080/api".to_string();

    let auth_url = std::env::var("AUTH_URL")
        .unwrap_or_else(|_| "https://scsys.jetbrains.space/oauth/auth".to_string());

    let token_url = std::env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://scsys.jetbrains.space/oauth/token".to_string());

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

async fn auth_jbspace(Extension(client): Extension<BasicClient>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("**".to_string()))
        .add_extra_param("access_type", "offline")
        .url();

    // Redirect to Google's oauth service
    Redirect::to(auth_url.as_ref())
}

// Valid user session required. If there is none, redirect to the auth page
pub async fn protected(user: User) -> impl IntoResponse {
    format!(
        "Welcome to the protected area :)\nHere's your info:\n{:?}",
        user
    )
}

async fn logout(
    Extension(store): Extension<MemoryStore>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> impl IntoResponse {
    let cookie = cookies.get(COOKIE_NAME).unwrap();
    let session = match store.load_session(cookie.to_string()).await.unwrap() {
        Some(s) => s,
        // No session active, just redirect
        None => return Redirect::to("/"),
    };

    store.destroy_session(session).await.unwrap();

    Redirect::to("/")
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

async fn login_authorized(
    Query(query): Query<AuthRequest>,
    Extension(store): Extension<MemoryStore>,
    Extension(oauth_client): Extension<BasicClient>,
) -> impl IntoResponse {
    // Get an auth token
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    // Fetch user data from Google
    let client = reqwest::Client::new();
    let user_data: User = client
        .get("https://scsys.jetbrains.space/api/http/organization")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();

    // Create a new session filled with user data
    let mut session = Session::new();
    session.insert("user", &user_data).unwrap();

    // Store session and get corresponding cookie
    let cookie = store.store_session(session).await.unwrap().unwrap();

    // Build the cookie
    let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    (headers, Redirect::to("/"))
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/jetbrains").into_response()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OAuth2Urls {
    pub auth: String,
    pub redirect: String,
    pub token: String,
}

impl OAuth2Urls {
    pub fn new(auth: String, redirect: String, token: String) -> Self {
        Self {
            auth,
            redirect,
            token,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OAuth2Client {
    #[serde(rename = "client_id")]
    pub id: String,
    #[serde(rename = "client_secret")]
    pub secret: String,
}

impl OAuth2Client {
    pub fn new(id: String, secret: String) -> Self {
        Self { id, secret }
    }
    pub fn from_env(id: Option<&str>, secret: Option<&str>) -> Self {
        let id = std::env::var(id.unwrap_or("CLIENT_ID")).unwrap_or_default();
        let secret = std::env::var(secret.unwrap_or("CLIENT_SECRET")).unwrap_or_default();
        Self::new(id, secret)
    }
}
