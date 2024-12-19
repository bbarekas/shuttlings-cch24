// Challenge 19 : https://console.shuttle.dev/shuttlings/cch24/challenge/19

use axum::{
    response::{IntoResponse, Json},
    routing::*,
    Router
};
use axum::extract::{State, Path, Query};
use axum::http::StatusCode;

use sqlx::PgPool;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Quote {
    id: uuid::Uuid,
    author: String,
    quote: String,
    created_at: chrono::DateTime<chrono::Utc>,
    version: i32,
}

#[derive(Debug, Deserialize)]
struct Draft {
    author: String,
    quote: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn get_routes(pool: PgPool) -> Router {
    let state = AppState { pool };
    Router::new()
        .route("/19/reset", post(handle_reset))
        .route("/19/cite/:id", get(handle_cite))
        .route("/19/remove/:id", delete(handle_remove))
        .route("/19/undo/:id", put(handle_undo))
        .route("/19/draft", post(handle_draft))
        //.route("/19/list", get(handle_list))
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
