// Challenge 2 : https://console.shuttle.dev/shuttlings/cch24/challenge/2

use std::net::{Ipv4Addr, Ipv6Addr};
use axum::{
    response::IntoResponse,
    routing::get,
    Router
};
use axum::extract::Query;
use serde::Deserialize;

pub fn get_routes() -> Router {
    Router::new()
        .route("/2/dest", get(handle_dest))
        .route("/2/key", get(handle_key))
        .route("/2/v6/dest", get(handle_dest_v6))
        .route("/2/v6/key", get(handle_key_v6))
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

    let a:u8 = from.octets().to_vec()[0].wrapping_add(key.octets().to_vec()[0]);
    let b:u8 = from.octets().to_vec()[1].wrapping_add(key.octets().to_vec()[1]);
    let c:u8 = from.octets().to_vec()[2].wrapping_add(key.octets().to_vec()[2]);
    let d:u8 = from.octets().to_vec()[3].wrapping_add(key.octets().to_vec()[3]);

    let destination = Ipv4Addr::new(a, b, c, d);
    destination.to_string()
}

#[derive(Debug, Deserialize)]
struct KeyParams {
    from: Option<Ipv4Addr>,
    to: Option<Ipv4Addr>,
}

async fn handle_key(params: Query<KeyParams>) -> impl IntoResponse {
    // Parse query parameters.
    let from = params.from.unwrap_or(Ipv4Addr::new(0, 0, 0, 0));
    let to = params.to.unwrap_or(Ipv4Addr::new(0, 0, 0, 0));

    let a:u8 = to.octets().to_vec()[0].wrapping_sub(from.octets().to_vec()[0]);
    let b:u8 = to.octets().to_vec()[1].wrapping_sub(from.octets().to_vec()[1]);
    let c:u8 = to.octets().to_vec()[2].wrapping_sub(from.octets().to_vec()[2]);
    let d:u8 = to.octets().to_vec()[3].wrapping_sub(from.octets().to_vec()[3]);

    let key = Ipv4Addr::new(a, b, c, d);
    key.to_string()
}

#[derive(Debug, Deserialize)]
struct Destv6Params {
    from: Option<Ipv6Addr>,
    key: Option<Ipv6Addr>,
}

async fn handle_dest_v6(params: Query<Destv6Params>) -> impl IntoResponse {
    // Parse query parameters.
    let from = params.from.unwrap_or(Ipv6Addr::new(0,0,0,0,0,0,0,0));
    let key = params.key.unwrap_or(Ipv6Addr::new(0,0,0,0,0,0,0,0));
    let from_parts = from.segments().to_vec();
    let key_parts = key.segments().to_vec();

    let a:u16 = from_parts[0] ^ key_parts[0];
    let b:u16 = from_parts[1] ^ key_parts[1];
    let c:u16 = from_parts[2] ^ key_parts[2];
    let d:u16 = from_parts[3] ^ key_parts[3];
    let e:u16 = from_parts[4] ^ key_parts[4];
    let f:u16 = from_parts[5] ^ key_parts[5];
    let g:u16 = from_parts[6] ^ key_parts[6];
    let h:u16 = from_parts[7] ^ key_parts[7];

    let destination = Ipv6Addr::new(a, b, c, d, e, f, g, h);
    destination.to_string()
}


#[derive(Debug, Deserialize)]
struct Keyv6Params {
    from: Option<Ipv6Addr>,
    to: Option<Ipv6Addr>,
}

async fn handle_key_v6(params: Query<Keyv6Params>) -> impl IntoResponse {
    // Parse query parameters.
    let from = params.from.unwrap_or(Ipv6Addr::new(0,0,0,0,0,0,0,0));
    let to = params.to.unwrap_or(Ipv6Addr::new(0,0,0,0,0,0,0,0));
    let from_parts = from.segments().to_vec();
    let to_parts = to.segments().to_vec();

    let a:u16 = from_parts[0] ^ to_parts[0];
    let b:u16 = from_parts[1] ^ to_parts[1];
    let c:u16 = from_parts[2] ^ to_parts[2];
    let d:u16 = from_parts[3] ^ to_parts[3];
    let e:u16 = from_parts[4] ^ to_parts[4];
    let f:u16 = from_parts[5] ^ to_parts[5];
    let g:u16 = from_parts[6] ^ to_parts[6];
    let h:u16 = from_parts[7] ^ to_parts[7];

    let destination = Ipv6Addr::new(a, b, c, d, e, f, g, h);
    destination.to_string()
}
