use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub server: server::ServerConfig,
    #[serde(default)]
    pub proxy: proxy::ProxyConfig,
    #[serde(default = "default_database_path")]
    pub database_path: PathBuf,
}

fn default_database_path() -> PathBuf {
    PathBuf::from("nodescope.db")
}

impl Config {
    pub fn init(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let config_file = std::fs::read_to_string(&path)
            .context(format!("Couldn't read config file {:?}", path.as_ref()))?;

        let config: Config =
            serde_yaml::from_str(&config_file).context("Couldn't parse config file")?;

        Ok(config)
    }
}
