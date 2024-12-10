use std::{sync::{Arc, RwLock}};

use axum::{
    Router,
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    extract::{State}
    
};
use leaky_bucket::RateLimiter;
use serde_json::json;

#[derive(Clone)]
pub struct AppState {
    limiter: Arc<RwLock<RateLimiter>>,
}

pub fn get_routes() -> Router {

    let limiter = RateLimiter::builder()
        .interval(std::time::Duration::from_secs(1))
        .max(5)
        .initial(5)
        .build();
    let limiter = Arc::new(RwLock::new(limiter));
    let state = AppState{ limiter };
    
    Router::new()
        .route("/9/milk", post(handle_milk))
        .route("/9/refill", post(handle_refill))
        .with_state(state)
}

#[derive(serde::Deserialize, Debug)]
struct Volume {
    gallons: Option<f32>,
    liters: Option<f32>,
    litres: Option<f32>,
    pints: Option<f32>,
}

async fn handle_milk(State(state): State<AppState>, headers: HeaderMap, body: String) ->  impl IntoResponse {
    if !state.limiter.read().unwrap().try_acquire(1) {
        return (StatusCode::TOO_MANY_REQUESTS, "No milk available\n").into_response()
    }
    if let Some("application/json") = headers.get(CONTENT_TYPE).and_then(|v| v.to_str().ok()) {
        let volume = serde_json::from_str::<Volume>(&body)
            .map_err(|_e| StatusCode::BAD_REQUEST.into_response());
        if volume.is_err() {
            return (StatusCode::BAD_REQUEST, "Bad JSON".to_string()).into_response()
        }
        //println!("Volume: {:?}", volume);
        let volume = volume.unwrap();
        match (volume.gallons, volume.liters, volume.litres, volume.pints) {
            // Gallons 
            (Some(gallons), None, None, None) => {
                let liters = gallons * 3.78541;
                (StatusCode::OK, json!({"liters": liters}).to_string()).into_response()
            },
            // Liters
            (None, Some(liters), None, None) => {
                let gallons = liters / 3.78541;
                (StatusCode::OK, json!({"gallons": gallons}).to_string()).into_response()
            }
            // UK Litres
            (None, None, Some(litres), None) => {
                let pints = litres * 1.759754;
                (StatusCode::OK, json!({"pints": pints}).to_string()).into_response()
            }
            // UK Pints
            (None, None, None, Some(pints)) => {
                let litres = pints / 1.759754;
                (StatusCode::OK, json!({"litres": litres}).to_string()).into_response()
            }
            _ => StatusCode::BAD_REQUEST.into_response()
        }
    } else {
        (StatusCode::OK, "Milk withdrawn\n").into_response()
    }
}

async fn handle_refill(State(state): State<AppState>) ->  impl IntoResponse {
    // Create a new bucket.
    let limiter = RateLimiter::builder()
        .interval(std::time::Duration::from_secs(1))
        .max(5)
        .initial(5)
        .build();
    // Update state. 
    *state.limiter.write().unwrap() = limiter;
    StatusCode::OK.into_response()
}
