use config::{ConfigError, File};
use config::Config as Configuration;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub host: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    development,
    test,
    production,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: Server,
    pub log: Log,
    pub env: ENV,
}

const CONFIG_FILE_PATH: &str = "./src/config/base.toml";
const CONFIG_FILE_PREFIX: &str = "./src/config/";

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());
        let mut s = Configuration::new();
        s.set("env", env.clone())?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        s.try_into()
    }
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::development => write!(f, "development"),
            ENV::test => write!(f, "test"),
            ENV::production => write!(f, "production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "test" => ENV::test,
            "production" => ENV::production,
            _ => ENV::development,
        }
    }
}
