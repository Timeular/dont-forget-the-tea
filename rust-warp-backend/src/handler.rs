use crate::{error::Error::*, WebResult};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};
use warp::{reject, reply::json, Reply};

#[derive(Serialize, Deserialize)]
pub struct ShoppingListRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct ShoppingList {
    pub id: i64,
    pub name: String,
}

pub async fn health_handler(db_pool: SqlitePool) -> WebResult<impl Reply> {
    sqlx::query("SELECT 1")
        .execute(&db_pool)
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok("OK")
}

pub async fn create_list(body: ShoppingListRequest, db_pool: SqlitePool) -> WebResult<impl Reply> {
    sqlx::query("INSERT INTO shopping_list (name) VALUES ($1)")
        .bind(body.name)
        .execute(&db_pool)
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;

    Ok("OK")
}

pub async fn get_lists(db_pool: SqlitePool) -> WebResult<impl Reply> {
    let mut cursor = sqlx::query("SELECT * FROM shopping_list")
        .map(|row: SqliteRow| ShoppingList {
            id: row.get(0),
            name: row.get(1),
        })
        .fetch(&db_pool);
    let mut result: Vec<ShoppingList> = Vec::new();
    while let Some(row) = cursor.next().await {
        result.push(row.map_err(|e| reject::custom(DBQueryError(e)))?);
    }
    Ok(json::<Vec<ShoppingList>>(&result))
}