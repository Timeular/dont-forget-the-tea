use crate::data::*;
use crate::{error::Error::*, Result, WebResult};
use futures::StreamExt;
use log::debug;
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
    debug!("Get Shopping Lists");
    let mut cursor = sqlx::query("SELECT * FROM shopping_list ORDER BY created_at DESC")
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
    let mapped: Vec<ShoppingListResponse> = result
        .into_iter()
        .map(|r| ShoppingListResponse {
            id: r.id,
            name: r.name,
            created_at: r.created_at,
        })
        .collect();
    Ok(json::<Vec<ShoppingListResponse>>(&mapped))
}

pub async fn get_list(list_id: i64, db_pool: SqlitePool) -> WebResult<impl Reply> {
    debug!("Get Shopping List");
    let mut cursor = sqlx::query("SELECT * FROM shopping_list WHERE id = $1")
        .bind(list_id)
        .map(|row: SqliteRow| ShoppingList {
            id: row.get(0),
            name: row.get(1),
            created_at: row.get(2),
            items: vec![],
        })
        .fetch(&db_pool);
    let mut results: Vec<ShoppingList> = Vec::new();
    while let Some(row) = cursor.next().await {
        results.push(row.map_err(|e| reject::custom(DBQueryError(e)))?);
    }

    let shopping_list = match results.get(0) {
        Some(v) => v,
        None => return Err(reject::custom(NoItemFoundError)),
    };

    let shopping_items = fetch_items_for_list(list_id, &db_pool)
        .await
        .map_err(|e| reject::custom(e))?;

    let res = ShoppingListDetailResponse {
        id: shopping_list.id,
        name: shopping_list.name.clone(),
        created_at: shopping_list.created_at.clone(),
        items: shopping_items
            .into_iter()
            .map(|i| ShoppingItemResponse {
                id: i.id,
                name: i.name,
                checked: i.checked,
                quantity: i.quantity,
            })
            .collect(),
    };

    Ok(json::<ShoppingListDetailResponse>(&res))
}

async fn fetch_items_for_list(list_id: i64, db_pool: &SqlitePool) -> Result<Vec<ShoppingItem>> {
    let mut cursor = sqlx::query(
        "SELECT id, name, checked, quantity FROM shopping_list_item WHERE shopping_list_id = $1",
    )
    .bind(list_id)
    .map(|row: SqliteRow| ShoppingItem {
        id: row.get(0),
        name: row.get(1),
        checked: row.get(2),
        quantity: row.get(3),
    })
    .fetch(db_pool);
    let mut result: Vec<ShoppingItem> = Vec::new();
    while let Some(row) = cursor.next().await {
        result.push(row.map_err(|e| DBQueryError(e))?);
    }
    Ok(result)
}

pub async fn create_item(
    list_id: i64,
    body: ShoppingItemRequest,
    db_pool: SqlitePool,
) -> WebResult<impl Reply> {
    debug!("Create Shopping List Item");
    sqlx::query(
        "INSERT INTO shopping_list_item(shopping_list_id, name, quantity) VALUES ($1, $2, $3)",
    )
    .bind(list_id)
    .bind(body.name)
    .bind(body.quantity)
    .execute(&db_pool)
    .await
    .map_err(|e| reject::custom(DBQueryError(e)))?;

    Ok("OK")
}
