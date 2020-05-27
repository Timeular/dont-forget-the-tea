#[macro_use]
extern crate lazy_static;
use log::info;
use settings::logging;
use sqlx::SqlitePool;
use std::convert::Infallible;
use std::fs;
use warp::{Filter, Rejection};

mod data;
mod error;
mod handler;
mod settings;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

const INIT_SQL: &str = "./db/init.sql";

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[tokio::main]
async fn main() -> Result<()> {
    logging::init(&CONFIG.log.level);

    let pool = SqlitePool::builder()
        .max_size(1)
        .build(&CONFIG.db.connect_str)
        .await?;

    if CONFIG.app.init_db {
        info!("Initializing DB...");
        init_db(&pool).await?;
        info!("DB initialized!");
    }

    let health_route = warp::path!("health")
        .and(with_db(pool.clone()))
        .and_then(handler::health_handler);

    let list_post = warp::post()
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(handler::create_list);

    let lists_get = warp::get()
        .and(with_db(pool.clone()))
        .and_then(handler::get_lists);

    let list_get = warp::get()
        .and(warp::path::param())
        .and(with_db(pool.clone()))
        .and_then(handler::get_list);

    let item = warp::path::param().and(warp::path("item"));
    let list_items_post = item
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(handler::create_item);

    let list_routes =
        warp::path("list").and(list_items_post.or(list_get).or(lists_get).or(list_post));

    let routes = health_route
        .or(list_routes)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    info!("Started server at localhost:8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(
    db_pool: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

async fn init_db(db_pool: &SqlitePool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    sqlx::query(&init_file)
        .execute(db_pool)
        .await
        .map_err(|e| error::Error::DBQueryError(e))?;
    Ok(())
}
