mod challenges;

use axum::{
    routing::get,
    Router
};
use sqlx::PgPool;
use shuttle_runtime::CustomError;

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    
    dotenv::dotenv().ok();
    
    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;
    
    // Merge routes from all the challenges. 
    let router = Router::new()
        .route("/", get(hello_world))
        .merge(challenges::challenge0::get_routes())
        .merge(challenges::challenge2::get_routes())
        .merge(challenges::challenge5::get_routes())
        .merge(challenges::challenge9::get_routes())
        .merge(challenges::challenge12::get_routes())
        .merge(challenges::challenge16::get_routes())
        .merge(challenges::challenge19::get_routes(pool))
        .merge(challenges::challenge23::get_routes());

    Ok(router.into())
}
