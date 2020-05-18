pub mod logging;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub connect_str: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct App {
    pub init_db: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub db: Database,
    pub log: Log,
    pub app: App,
}

const CONFIG_FILE_PATH: &str = "./config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        let mut s = Config::new();
        s.set("env", env.clone())?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        s.merge(Environment::with_prefix("dftt").separator("__"))?;

        s.try_into()
    }
}
