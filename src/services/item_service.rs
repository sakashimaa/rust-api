use crate::models::{Item, RequestItem};
use sqlx::PgPool;

#[derive(Clone)]
pub struct ItemService {
    db_pool: PgPool,
}

impl ItemService {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create_item(&self, request: &RequestItem) -> Result<Item, sqlx::Error> {
        let query = r#"
            INSERT INTO items (name, description)
            VALUES ($1, $2)
            RETURNING id, name, description
        "#;

        let row: (i32, String, String) = sqlx::query_as(query)
            .bind(&request.name)
            .bind(&request.description)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(Item {
            id: row.0,
            name: row.1,
            description: row.2,
        })
    }

    pub async fn get_items(&self) -> Result<Vec<Item>, sqlx::Error> {
        let query = "SELECT * FROM items";

        sqlx::query_as::<_, Item>(query)
            .fetch_all(&self.db_pool)
            .await
    }

    pub async fn get_item(&self, id: i32) -> Result<Option<Item>, sqlx::Error> {
        let query = "SELECT * FROM items WHERE id = $1";

        sqlx::query_as::<_, Item>(query)
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await
    }

    pub async fn update_item(
        &self,
        id: i32,
        request: &RequestItem,
    ) -> Result<Option<Item>, sqlx::Error> {
        let query = r#"
            UPDATE items
            SET name = $1, description = $2
            WHERE id = $3
            RETURNING id, name, description
        "#;

        sqlx::query_as::<_, Item>(query)
            .bind(&request.name)
            .bind(&request.description)
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await
    }

    pub async fn delete_item(&self, id: i32) -> Result<bool, sqlx::Error> {
        let query = "DELETE FROM items WHERE id = $1";

        let result = sqlx::query(query).bind(id).execute(&self.db_pool).await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_all_items(&self) -> Result<u64, sqlx::Error> {
        let query = "DELETE FROM items";

        let result = sqlx::query(query).execute(&self.db_pool).await?;

        Ok(result.rows_affected())
    }
}
