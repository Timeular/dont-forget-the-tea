use serde_derive::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Docker,
    Production,
    IntegrationTests,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "development"),
            ENV::Testing => write!(f, "testing"),
            ENV::Docker => write!(f, "docker"),
            ENV::Production => write!(f, "production"),
            ENV::IntegrationTests => write!(f, "integrationTests"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "testing" => ENV::Testing,
            "production" => ENV::Production,
            "docker" => ENV::Docker,
            "integrationTests" => ENV::IntegrationTests,
            _ => ENV::Development,
        }
    }
}
