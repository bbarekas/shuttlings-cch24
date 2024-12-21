// Challenge 19 : https://console.shuttle.dev/shuttlings/cch24/challenge/19

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::{
    response::{IntoResponse, Json},
    routing::*,
    Router
};
use axum::extract::{State, Path, Query};
use axum::http::StatusCode;
use sqlx::{PgPool, FromRow};
use serde::{Deserialize, Serialize};
use rand::{
    distributions::{Alphanumeric, DistString},
};

#[derive(Debug, FromRow, Deserialize, Serialize)]

struct Quote {
    id: uuid::Uuid,
    author: String,
    quote: String,
    created_at: chrono::DateTime<chrono::Utc>,
    version: i64,
}

#[derive(Debug, Deserialize)]
struct Draft {
    author: String,
    quote: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    tokens: Arc<Mutex<HashMap<String, i32>>>,
}

pub fn get_routes(pool: PgPool) -> Router {
    
    let tokens = Arc::new(Mutex::new(HashMap::new()));
    let state = AppState { pool, tokens };
    Router::new()
        .route("/19/reset", post(handle_reset))
        .route("/19/cite/:id", get(handle_cite))
        .route("/19/remove/:id", delete(handle_remove))
        .route("/19/undo/:id", put(handle_undo))
        .route("/19/draft", post(handle_draft))
        .route("/19/list", get(handle_list))
        .with_state(state)
}

async fn handle_reset(State(state): State<AppState>) -> impl IntoResponse {
    // Parse query parameters.
    let pool = &state.pool;

    let _ = sqlx::query!("TRUNCATE quotes")
        .execute(pool)
        .await
        .unwrap();

    (StatusCode::OK, "Database reset.").into_response()
}

async fn handle_cite(State(state): State<AppState>, Path(uuid): Path<uuid::Uuid>) -> impl IntoResponse {
    let pool = &state.pool;
    let Ok(quote) = sqlx::query_as!(Quote, "SELECT * FROM quotes WHERE id = $1", uuid)
        .fetch_one(pool)
        .await
    else {
        return (StatusCode::NOT_FOUND, "Item not found.").into_response()
    };

    (StatusCode::OK, Json(quote).into_response()).into_response()
}

async fn handle_remove(State(state): State<AppState>, Path(uuid): Path<uuid::Uuid>) -> impl IntoResponse {
    let pool = &state.pool;
    let Ok(quote) = sqlx::query_as!(Quote, "DELETE FROM quotes WHERE id = $1 RETURNING *", uuid)
        .fetch_one(pool)
        .await
    else {
        return (StatusCode::NOT_FOUND, "Item not found.").into_response()
    };
    (StatusCode::OK, Json(quote)).into_response()
}

async fn handle_undo(State(state): State<AppState>, Path(uuid): Path<uuid::Uuid>, Json(draft): Json<Draft>) -> impl IntoResponse {
    let pool = &state.pool;
    let Ok(quote) = sqlx::query_as!(Quote, "UPDATE quotes SET author = $2, quote = $3, \
                                version = version +1  WHERE id = $1 RETURNING *",
                                uuid, draft.author, draft.quote)
        .fetch_one(pool)
        .await
    else {
        return (StatusCode::NOT_FOUND, "Item not found.").into_response()
    };
    (StatusCode::OK, Json(quote)).into_response()
}

async fn handle_draft(State(state): State<AppState>,  Json(draft): Json<Draft>) -> impl IntoResponse {
    let pool = &state.pool;
    let uuid = uuid::Uuid::new_v4();
    let Ok(quote) = sqlx::query_as!(Quote, "INSERT INTO quotes (id, author, quote) \
                                VALUES ($1, $2, $3) RETURNING *",
                                uuid, draft.author, draft.quote )
        .fetch_one(pool)
        .await
    else {
        return (StatusCode::NOT_FOUND, "Item not found.").into_response()
    };
    (StatusCode::CREATED, Json(quote)).into_response()
}

#[derive(Debug, Deserialize, Serialize)]
struct Token {
    token: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct List {
    quotes: Vec<Quote>,
    page: i32,
    next_token: Option<String>,
}

async fn handle_list(
    State(state): State<AppState>,
    token: Option<Query<Token>>,
) -> Result<Json<List>, StatusCode> {
    // Check if token match a page.
    let page = if let Some(Query(token)) = token {
        let map = state.tokens.lock().unwrap();
        let number = map.get(&token.token).map(|i| *i).ok_or(StatusCode::BAD_REQUEST)?;
        number
    } else {
        0
    };
    let quotes= sqlx::query_as!(Quote, "SELECT * FROM quotes ORDER BY created_at ASC LIMIT 4 OFFSET $1", 
                    (page*3) as i64)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let next_token = if quotes.len() == 4 {
        let new_token = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        state.tokens.lock().unwrap().insert(new_token.clone(), page + 1);
        Some(new_token)
    } else {
        None
    };
    let quotes = quotes.into_iter().take(3).collect();
    Ok(Json(List {
        quotes,
        page: page + 1,
        next_token,
    }))
}
