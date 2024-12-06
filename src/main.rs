mod challenges;

use axum::{
    routing::get,
    Router
};

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .merge(challenges::challenge0::get_routes())
        .merge(challenges::challenge2::get_routes());

    Ok(router.into())
}
