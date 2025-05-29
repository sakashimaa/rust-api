mod config;
mod database;
mod handlers;
mod models;
mod routes;
mod services;

use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config = config::load_config();
    let db_pool = database::create_pool(&config.database_url).await;

    let app = Router::new().merge(routes::item_routes(db_pool.clone()));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
