use axum::{
    routing::get,
    extract::Query,
    http::{StatusCode},
    response::{IntoResponse},
    Router
};
use std::net::Ipv4Addr;
use serde::Deserialize;

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

async fn handle_redirect() -> impl IntoResponse {
    return (
        StatusCode::FOUND,
        [("location", "https://www.youtube.com/watch?v=9Gc4QTqslN4")],
    );
    // This returns 303 ...
    // Redirect::to("https://www.youtube.com/watch?v=9Gc4QTqslN4").into_response()
}

#[derive(Debug, Deserialize)]
struct DestParams {
    from: Option<Ipv4Addr>,
    key: Option<Ipv4Addr>,
}

async fn handle_dest(params: Query<DestParams>) -> impl IntoResponse {

    // Parse query parameters.
    let from = params.from.unwrap_or(Ipv4Addr::new(0, 0, 0, 0));
    let key = params.key.unwrap_or(Ipv4Addr::new(0, 0, 0, 0));

    //println!("From: {}  Key: {}", from, key);
    let a:u8 = from.octets().to_vec()[0].wrapping_add(key.octets().to_vec()[0]);
    let b:u8 = from.octets().to_vec()[1].wrapping_add(key.octets().to_vec()[1]);
    let c:u8 = from.octets().to_vec()[2].wrapping_add(key.octets().to_vec()[2]);
    let d:u8 = from.octets().to_vec()[3].wrapping_add(key.octets().to_vec()[3]);
    //println!("From: {:?} {:?} {:?} {:?}", a, b, c, d);

    let destination = Ipv4Addr::new(a, b, c, d);
    destination.to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(handle_redirect))
        .route("/2/dest", get(handle_dest));

    Ok(router.into())
}
