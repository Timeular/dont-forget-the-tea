use crate::Request;
use log::info;

pub mod grocery;
mod util;

static HEALTH: &str = "OK";

pub async fn health(req: Request) -> &'static str {
    let app_state = req.state();
    info!("sample_state: {}", app_state.sample_state);
    HEALTH
}
