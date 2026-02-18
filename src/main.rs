mod btcnode_metrics_gatherer;
mod handlers;
mod state;

use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use clap::Parser;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::btcnode_metrics_gatherer::{AppConfig, BitcoinMetrics, BitcoinNode, MetricsCollector, MetricsService};

use crate::state::AppState;

#[derive(Parser)]
#[command(name = "btc-metrics", about = "Bitcoin node metrics exporter for Prometheus")]
struct Cli {
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let cli = Cli::parse();
    let config = AppConfig::load(&cli.config)?;

    info!(rpc_url = %config.node.rpc_url, "Connecting to Bitcoin node");

    let node = BitcoinNode::new(&config.node)?;
    let metrics = BitcoinMetrics::new()?;
    let collector = MetricsCollector::new(node, metrics);
    let service = Arc::new(MetricsService::new(collector));

    let state = AppState { service };

    let app = Router::new()
        .route("/metrics", get(handlers::metrics_handler))
        .route("/health", get(handlers::health_handler))
        .with_state(state);

    let listener = TcpListener::bind(&config.server.listen_addr).await?;
    info!(addr = %config.server.listen_addr, "Listening for Prometheus scrapes");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    info!("Shutdown signal received");
}
