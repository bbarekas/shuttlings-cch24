// Challenge 23 : https://console.shuttle.dev/shuttlings/cch24/challenge/23

use axum::{
    response::{IntoResponse},
    routing::*,
    Router,
    extract::{Path, Multipart},
    http::StatusCode,
};
use tower_http::services::ServeDir;
use tera::escape_html;
use serde::Deserialize;

pub fn get_routes() -> Router {
    Router::new()
        .nest_service("/assets/", ServeDir::new("assets"))
        .route("/23/star", get(handle_star))
        .route("/23/present/:color", get(handle_present))
        .route("/23/ornament/:state/:n", get(handle_ornament))
        .route("/23/lockfile", post(handle_lockfile))
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

#[derive(Deserialize, Debug)]
struct Lockfile {
    package: Vec<Package>,
}

#[derive(Deserialize, Debug)]
struct Package {
    checksum: Option<String>,
}

async fn handle_lockfile(mut multipart: Multipart) -> impl IntoResponse {
    let mut part_data = String::new();
    while let Ok(Some(part)) = multipart.next_field().await {
        if part.name() == Some("lockfile") {
            part_data.push_str(&part.text().await.unwrap());
        }
    }
    let lockfile: Lockfile = match toml::de::from_str(&part_data) {
        Ok(res) => res,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, e.message().to_string()).into_response()
        },
    };
    let mut response = Vec::new();
    for package in lockfile.package {
        if let Some(checksum) = package.checksum {
            if checksum.len() < 10 {
                return (StatusCode::UNPROCESSABLE_ENTITY, "Not enough characters").into_response()
            }
            let color = match u32::from_str_radix(&checksum[..6], 16) {
                Ok(c) => format!("#{:06x}", c),
                Err(_) => return (StatusCode::UNPROCESSABLE_ENTITY, "Cannot get color value").into_response()
            };
            let top = match u8::from_str_radix(&checksum[6..8], 16) {
                Ok(t) => t,
                Err(_) => return (StatusCode::UNPROCESSABLE_ENTITY, "Cannot get top value").into_response()
            };
            let left = match u8::from_str_radix(&checksum[8..10], 16) {
                Ok(l) => l,
                Err(_) => return (StatusCode::UNPROCESSABLE_ENTITY, "Cannot get left value").into_response()
            };
            let div_element = format!(r#"<div style="background-color:{color};top:{top}px;left:{left}px;"></div>"#);
            response.push(div_element);
        }
    }
    (StatusCode::OK, response.join("\n")).into_response()
}