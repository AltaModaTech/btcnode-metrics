use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Bitcoin RPC error: {0}")]
    Rpc(#[from] corepc_client::client_sync::Error),

    #[error("Prometheus error: {0}")]
    Prometheus(#[from] prometheus::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}
