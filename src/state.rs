use std::sync::Arc;

use crate::btcnode_metrics_gatherer::{BitcoinNode, MetricsService};

pub struct AppState {
    pub service: Arc<MetricsService<BitcoinNode>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            service: Arc::clone(&self.service),
        }
    }
}
