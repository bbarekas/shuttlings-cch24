use axum::{
    routing::get,
    http::{StatusCode},
    response::{IntoResponse},
    Router
};

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


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(handle_redirect));

    Ok(router.into())
}
