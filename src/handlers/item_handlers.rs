use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json as AxumJson},
};

use crate::models::{DeleteAllResponse, RequestItem};
use crate::services::ItemService;

pub async fn root() -> &'static str {
    "Items API!"
}

pub async fn create_item(
    State(service): State<ItemService>,
    Json(payload): Json<RequestItem>,
) -> impl IntoResponse {
    match service.create_item(&payload).await {
        Ok(item) => (StatusCode::CREATED, AxumJson(item)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_items(State(service): State<ItemService>) -> impl IntoResponse {
    match service.get_items().await {
        Ok(items) => AxumJson(items).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_item(
    State(service): State<ItemService>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match service.get_item(id).await {
        Ok(Some(item)) => (StatusCode::OK, AxumJson(item)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_item(
    State(service): State<ItemService>,
    Path(id): Path<i32>,
    Json(payload): Json<RequestItem>,
) -> impl IntoResponse {
    match service.update_item(id, &payload).await {
        Ok(Some(item)) => (StatusCode::OK, AxumJson(item)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_item(
    State(service): State<ItemService>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match service.delete_item(id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_all(State(service): State<ItemService>) -> impl IntoResponse {
    match service.delete_all_items().await {
        Ok(count) => (StatusCode::OK, AxumJson(DeleteAllResponse { count })).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
