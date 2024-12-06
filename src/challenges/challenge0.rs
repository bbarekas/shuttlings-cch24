// Challenge -1 : https://console.shuttle.dev/shuttlings/cch24/challenge/-1

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router
};

pub fn get_routes() -> Router {
    Router::new()
        .route("/-1/seek", get(handle_redirect))

}

async fn handle_redirect() -> impl IntoResponse {
    return (
        StatusCode::FOUND,
        [("location", "https://www.youtube.com/watch?v=9Gc4QTqslN4")],
    );
    // This returns 303 ...
    // Redirect::to("https://www.youtube.com/watch?v=9Gc4QTqslN4").into_response()
}
