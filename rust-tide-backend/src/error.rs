use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not generate config: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("error parsing JSON: {0}")]
    ParseJSONError(#[from] serde_json::Error),
    #[error("server finished with an error: {0}")]
    ServerCrashedError(std::io::Error),
    #[error("could not parse JSON Body: {0}")]
    ParseJSONBodyError(std::io::Error),
}
