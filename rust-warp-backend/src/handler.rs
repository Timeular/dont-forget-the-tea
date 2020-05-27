use crate::data::{ShoppingList, ShoppingListRequest, ShoppingItemRequest};
use crate::{error::Error::*, WebResult};
use log::debug;
use futures::StreamExt;
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};
use warp::{reject, reply::json, Reply};

pub async fn health_handler(db_pool: SqlitePool) -> WebResult<impl Reply> {
    sqlx::query("SELECT 1")
        .execute(&db_pool)
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok("OK")
}

pub async fn create_list(body: ShoppingListRequest, db_pool: SqlitePool) -> WebResult<impl Reply> {
    debug!("Create Shopping List");
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
            created_at: row.get(2),
            items: vec![],
        })
        .fetch(&db_pool);
    let mut result: Vec<ShoppingList> = Vec::new();
    while let Some(row) = cursor.next().await {
        result.push(row.map_err(|e| reject::custom(DBQueryError(e)))?);
    }
    Ok(json::<Vec<ShoppingList>>(&result))
}

pub async fn create_item(list_id: i64, body: ShoppingItemRequest, db_pool: SqlitePool) -> WebResult<impl Reply> {
    debug!("Create Shopping List Item");
    sqlx::query("INSERT INTO shopping_list_item(shopping_list_id, name, quantity) VALUES ($1, $2, $3)")
        .bind(list_id)
        .bind(body.name)
        .bind(body.quantity)
        .execute(&db_pool)
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
 
    Ok("OK")
}