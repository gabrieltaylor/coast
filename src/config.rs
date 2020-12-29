use config::{ConfigError, File};
use config::Config as Configuration;
use serde::Deserialize;
use std::fmt;
use tide::log::LevelFilter;

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    #[serde(with = "LevelFilterDef")]
    pub level: LevelFilter,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(remote = "LevelFilter")]
#[serde(rename_all = "lowercase")]
pub enum LevelFilterDef {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub host: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ENV {
    Development,
    Test,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "development"),
            ENV::Test => write!(f, "test"),
            ENV::Production => write!(f, "production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "test" => ENV::Test,
            "production" => ENV::Production,
            _ => ENV::Development,
        }
    }
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
        let mut c = Configuration::new();
        c.set("env", env.clone())?;

        c.merge(File::with_name(CONFIG_FILE_PATH))?;
        c.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        c.try_into()
    }
}
