pub mod env;
pub mod logging;

use config::{Config, ConfigError, Environment, File};
use env::ENV;
use serde_derive::Deserialize;
use std::env as StdEnv;

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub log: Log,
    pub env: ENV,
}

const CONFIG_FILE_PATH: &str = "config/default.toml";
const CONFIG_FILE_PREFIX: &str = "config/";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = StdEnv::var("RUN_ENV").unwrap_or_else(|_| "development".into());
        let mut s = Config::new();
        s.set("env", env.clone())?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        // This makes it so "RTB_SERVER__PORT overrides the server.port key for example
        // the __ is used, because variables might be named some_var
        s.merge(Environment::with_prefix("rtb").separator("__"))?;

        s.try_into()
    }
}
