use serde::Deserialize;
use std::path::Path;

use super::Error;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub node: NodeConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub listen_addr: String,
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self, Error> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| Error::Config(format!("failed to read config file: {e}")))?;

        let mut config: AppConfig = toml::from_str(&contents)
            .map_err(|e| Error::Config(format!("failed to parse config: {e}")))?;

        // Environment variable overrides
        if let Ok(val) = std::env::var("BTC_METRICS_RPC_URL") {
            config.node.rpc_url = val;
        }
        if let Ok(val) = std::env::var("BTC_METRICS_RPC_USER") {
            config.node.rpc_user = val;
        }
        if let Ok(val) = std::env::var("BTC_METRICS_RPC_PASSWORD") {
            config.node.rpc_password = val;
        }
        if let Ok(val) = std::env::var("BTC_METRICS_LISTEN_ADDR") {
            config.server.listen_addr = val;
        }

        Ok(config)
    }
}
