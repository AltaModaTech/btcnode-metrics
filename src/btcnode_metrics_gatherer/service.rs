use super::{MetricsCollector, NodeClient};
use prometheus::Encoder;
use prometheus::TextEncoder;

pub struct MetricsService<N: NodeClient> {
    collector: MetricsCollector<N>,
}

impl<N: NodeClient> MetricsService<N> {
    pub fn new(collector: MetricsCollector<N>) -> Self {
        Self { collector }
    }

    pub fn scrape(&self) -> String {
        self.collector.collect();

        let encoder = TextEncoder::new();
        let metric_families = self.collector.metrics().registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).expect("encoding metrics should not fail");
        String::from_utf8(buffer).expect("prometheus text format is valid UTF-8")
    }
}
