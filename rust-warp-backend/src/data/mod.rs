use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShoppingListRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShoppingList {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub items: Vec<ShoppingItem>,
}

#[derive(Serialize, Deserialize)]
pub struct ShoppingItem {
    pub id: i64,
    pub name: String,
    pub checked: bool,
    pub quantity: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ShoppingItemRequest {
    pub name: String,
    pub quantity: Option<i32>,
}