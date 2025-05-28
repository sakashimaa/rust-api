use core::error;
use std::env;

use axum::response::{IntoResponse, Json as AxumJson};
use axum::routing::post;
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tokio::net::TcpListener;

#[derive(Serialize, FromRow)]
struct Item {
    id: i32,
    name: String,
    description: String,
}

#[derive(Serialize)]
struct DeleteAllResponse {
    count: u64,
}

#[derive(Deserialize)]
struct RequestItem {
    name: String,
    description: String,
}

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

impl AppState {
    async fn create_item(&self, name: &str, description: &str) -> Result<Item, sqlx::Error> {
        let query = r#"
            INSERT INTO items (name, description)
            VALUES ($1, $2)
            RETURNING id, name, description
        "#;

        let row: (i32, String, String) = sqlx::query_as(query)
            .bind(name)
            .bind(description)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(Item {
            id: row.0,
            name: row.1,
            description: row.2,
        })
    }
    async fn get_items(&self) -> Result<Vec<Item>, sqlx::Error> {
        let query = r#"
            SELECT * FROM items
        "#;

        let result = sqlx::query_as::<_, Item>(query)
            .fetch_all(&self.db_pool)
            .await?;

        Ok(result)
    }
    async fn get_item(&self, id: i32) -> Result<Option<Item>, sqlx::Error> {
        let query = r#"
            SELECT * FROM items WHERE id = $1
        "#;

        let result = sqlx::query_as::<_, Item>(query)
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;

        Ok(result)
    }
    async fn update_item(
        &self,
        id: i32,
        name: &str,
        description: &str,
    ) -> Result<Option<Item>, sqlx::Error> {
        let query = r#"
            UPDATE items
            SET name = $1, description = $2
            WHERE id = $3
            RETURNING id, name, description
        "#;

        let result = sqlx::query_as::<_, Item>(query)
            .bind(name)
            .bind(description)
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;

        Ok(result)
    }
    async fn delete_item(&self, id: i32) -> Result<bool, sqlx::Error> {
        let query = r#"
            DELETE FROM items
            WHERE id = $1
        "#;

        let result = sqlx::query(query).bind(id).execute(&self.db_pool).await?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_all_items(&self) -> Result<u64, sqlx::Error> {
        let query = r#"
            DELETE FROM items
        "#;

        let result = sqlx::query(query).execute(&self.db_pool).await?;

        Ok(result.rows_affected())
    }
}

async fn root() -> &'static str {
    "Items API!"
}

async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<RequestItem>,
) -> (StatusCode, AxumJson<Item>) {
    let item = state
        .create_item(&payload.name, &payload.description)
        .await
        .unwrap();

    (StatusCode::CREATED, AxumJson(item))
}

async fn get_items(State(state): State<AppState>) -> impl IntoResponse {
    AxumJson(state.get_items().await.unwrap())
}

async fn get_item(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    match state.get_item(id).await.unwrap() {
        Some(item) => (StatusCode::OK, AxumJson(item)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn update_item(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(payload): Json<RequestItem>,
) -> impl IntoResponse {
    match state
        .update_item(id, &payload.name, &payload.description)
        .await
        .unwrap()
    {
        Some(item) => (StatusCode::OK, AxumJson(item)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn delete_item(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    match state.delete_item(id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn delete_all(State(state): State<AppState>) -> impl IntoResponse {
    match state.delete_all_items().await {
        Ok(count) => (StatusCode::OK, AxumJson(DeleteAllResponse { count })).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in file");

    let db_pool = PgPool::connect(&db_url)
        .await
        .expect("failed to connect to postgres");

    let app = Router::new()
        .route("/", get(root))
        .route("/item", post(create_item))
        .route("/items", get(get_items).delete(delete_all))
        .route(
            "/items/{id}",
            get(get_item).delete(delete_item).put(update_item),
        )
        .with_state(AppState { db_pool });

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
