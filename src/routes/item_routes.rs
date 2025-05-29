use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;

use crate::handlers::item_handlers::*;
use crate::services::ItemService;

pub fn item_routes(db_pool: PgPool) -> Router {
    let item_service = ItemService::new(db_pool);

    Router::new()
        .route("/", get(root))
        .route("/item", post(create_item))
        .route("/items", get(get_items).delete(delete_all))
        .route(
            "/items/{id}",
            get(get_item).delete(delete_item).put(update_item),
        )
        .with_state(item_service)
}
