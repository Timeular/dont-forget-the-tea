#[macro_use]
extern crate lazy_static;
use app::{App, AppState};
use error::Error;

mod app;
mod data;
mod error;
mod handler;
mod settings;

type Result<T> = std::result::Result<T, Error>;
pub type Request = tide::Request<AppState>;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[async_std::main]
async fn main() -> Result<()> {
    let app = App::new()?;
    app.run().await
}
