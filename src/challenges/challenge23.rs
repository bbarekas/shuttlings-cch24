// Challenge 23 : https://console.shuttle.dev/shuttlings/cch24/challenge/23

use axum::{
    response::{IntoResponse},
    routing::*,
    Router,
    extract::{Path},
    http::StatusCode,
};
use tower_http::services::ServeDir;
use tera::escape_html;

pub fn get_routes() -> Router {
    Router::new()
        .nest_service("/assets/", ServeDir::new("assets"))
        .route("/23/star", get(handle_star))
        .route("/23/present/:color", get(handle_present))
        .route("/23/ornament/:state/:n", get(handle_ornament))
}

async fn handle_star() -> impl IntoResponse {
    (StatusCode::OK, "<div class=\"lit\" id=\"star\"></div>").into_response()
}

async fn handle_present(Path(color): Path<String>) -> impl IntoResponse {
    let next_color = match color.as_str() {
        "red" => "blue",
        "blue" => "purple",
        "purple" => "red",
        _ => return (StatusCode::IM_A_TEAPOT).into_response(),
    };
    (StatusCode::OK, format!("<div class=\"present {color}\" hx-get=\"/23/present/{next_color}\" hx-swap=\"outerHTML\"> \
                     <div class=\"ribbon\"></div>
                     <div class=\"ribbon\"></div>
                     <div class=\"ribbon\"></div>
                     <div class=\"ribbon\"></div>
                     </div>")).into_response()
}

async fn handle_ornament(Path((state, n)): Path<(String, String)>) -> impl IntoResponse {
    let n = escape_html(n.as_str());
    match state.as_str() {
        "on" => return (StatusCode::OK, format!("<div class=\"ornament on\" id=\"ornament{n}\" \
                        hx-trigger=\"load delay:2s once\" \
                        hx-get=\"/23/ornament/off/{n}\" hx-swap=\"outerHTML\"></div>")).into_response(),
        "off" => return (StatusCode::OK, format!("<div class=\"ornament\" id=\"ornament{n}\" \
                        hx-trigger=\"load delay:2s once\" \
                        hx-get=\"/23/ornament/on/{n}\" hx-swap=\"outerHTML\"></div>")).into_response(),
        _ => return (StatusCode::IM_A_TEAPOT).into_response(),
    };
}
