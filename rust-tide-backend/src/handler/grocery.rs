use super::util::HandlerResponse;
use crate::{data::grocery::*, error::Error::*, Request, Result};
use chrono::Utc;
use log::info;
use tide::Response;

pub async fn list(_req: Request) -> HandlerResponse {
    HandlerResponse::new(do_list().await)
}

pub async fn create_list(req: Request) -> HandlerResponse {
    HandlerResponse::new(do_create_list(req).await)
}

pub async fn do_list() -> Result<Response> {
    let grocery_list = GroceryList {
        name: String::from("example list"),
        creation_date: Utc::now(),
        items: vec![GroceryItem {
            name: String::from("tea"),
            amount: Some(1),
        }],
    };
    let res = Response::new(200).body_json(&grocery_list)?;
    Ok(res)
}

pub async fn do_create_list(mut req: Request) -> Result<Response> {
    let new_list: GroceryList = req.body_json().await.map_err(ParseJSONBodyError)?;
    info!("creating grocery list: {:?}", new_list);
    let res = Response::new(200).body_json(&new_list)?;
    Ok(res)
}
