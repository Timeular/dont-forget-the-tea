use serde::{Deserialize, Serialize};

pub struct ShoppingList {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub items: Vec<ShoppingItem>,
}

pub struct ShoppingItem {
    pub id: i64,
    pub name: String,
    pub checked: bool,
    pub quantity: Option<i32>,
}

#[derive(Deserialize)]
pub struct ShoppingListRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct ShoppingListResponse {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct ShoppingListDetailResponse {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub items: Vec<ShoppingItemResponse>,
}

#[derive(Serialize)]
pub struct ShoppingItemResponse {
    pub id: i64,
    pub name: String,
    pub checked: bool,
    pub quantity: Option<i32>,
}

#[derive(Deserialize)]
pub struct ShoppingItemRequest {
    pub name: String,
    pub quantity: Option<i32>,
}
