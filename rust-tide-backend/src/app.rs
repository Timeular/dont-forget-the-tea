use crate::{error::Error::*, handler, settings::logging, Result, CONFIG};
use log::info;

pub struct AppState {
    pub sample_state: String,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            sample_state: String::from("dummy state"),
        }
    }
}

pub struct App {}

impl App {
    pub fn new() -> Result<App> {
        logging::init(&CONFIG.log.level);
        Ok(App {})
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting with environment: {}", CONFIG.env);

        let app_state = AppState::new();

        let addr: String = format!("0.0.0.0:{}", CONFIG.server.port).parse().unwrap();
        let mut tide_app = tide::with_state(app_state);

        tide_app.at("/health").get(handler::health);
        tide_app.at("/grocery/list/").get(handler::grocery::list);
        tide_app
            .at("/grocery/list/")
            .post(handler::grocery::create_list);
        tide_app
            .listen(&addr)
            .await
            .map_err(|e| ServerCrashedError(e))?;

        Ok(())
    }
}
