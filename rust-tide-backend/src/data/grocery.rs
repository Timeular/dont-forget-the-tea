use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GroceryList {
    pub name: String,
    pub creation_date: DateTime<Utc>,
    pub items: Vec<GroceryItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GroceryItem {
    pub name: String,
    pub amount: Option<usize>,
}
