use std::time::Instant;

use tracing::{info, warn};

use super::metrics::BitcoinMetrics;
use super::node::NodeClient;

pub struct MetricsCollector<N: NodeClient> {
    node: N,
    metrics: BitcoinMetrics,
}

impl<N: NodeClient> MetricsCollector<N> {
    pub fn new(node: N, metrics: BitcoinMetrics) -> Self {
        Self { node, metrics }
    }

    pub fn metrics(&self) -> &BitcoinMetrics {
        &self.metrics
    }

    pub fn collect(&self) {
        let start = Instant::now();
        let mut had_error = false;
        let mut block_height: Option<i64> = None;

        // Blockchain info
        match self.node.get_blockchain_info() {
            Ok(info) => {
                self.metrics.blocks.set(info.blocks as f64);
                self.metrics.headers.set(info.headers as f64);
                self.metrics.difficulty.set(info.difficulty);
                self.metrics.verification_progress.set(info.verification_progress);
                self.metrics.size_on_disk.set(info.size_on_disk as f64);
                self.metrics.initial_block_download.set(if info.initial_block_download { 1.0 } else { 0.0 });
                self.metrics.chain_pruned.set(if info.pruned { 1.0 } else { 0.0 });
                block_height = Some(info.blocks);
                info!("Updated blockchain info: blocks={}, headers={}", info.blocks, info.headers);
            }
            Err(e) => {
                warn!("Failed to get blockchain info: {e}");
                had_error = true;
            }
        }

        // Mempool info
        match self.node.get_mempool_info() {
            Ok(info) => {
                self.metrics.mempool_transactions.set(info.size as f64);
                self.metrics.mempool_bytes.set(info.bytes as f64);
                self.metrics.mempool_usage.set(info.usage as f64);
                self.metrics.mempool_max_bytes.set(info.max_mempool as f64);
                self.metrics.mempool_min_fee.set(info.mempool_min_fee);
                self.metrics.mempool_total_fee.set(info.total_fee);
                self.metrics.mempool_min_relay_tx_fee.set(info.min_relay_tx_fee);
                self.metrics.mempool_incremental_relay_fee.set(info.incremental_relay_fee);
                self.metrics.mempool_unbroadcast_count.set(info.unbroadcast_count as f64);
                self.metrics.mempool_full_rbf.set(if info.full_rbf { 1.0 } else { 0.0 });
                info!("Updated mempool info: txs={}, bytes={}", info.size, info.bytes);
            }
            Err(e) => {
                warn!("Failed to get mempool info: {e}");
                had_error = true;
            }
        }

        // Network info
        match self.node.get_network_info() {
            Ok(info) => {
                self.metrics.connections.set(info.connections as f64);
                self.metrics.connections_in.set(info.connections_in as f64);
                self.metrics.connections_out.set(info.connections_out as f64);
                self.metrics.network_active.set(if info.network_active { 1.0 } else { 0.0 });
                self.metrics.node_version.set(info.version as f64);
                self.metrics.protocol_version.set(info.protocol_version as f64);
                self.metrics.time_offset.set(info.time_offset as f64);
                self.metrics.relay_fee.set(info.relay_fee);
                self.metrics.incremental_fee.set(info.incremental_fee);
                info!("Updated network info: connections={}", info.connections);
            }
            Err(e) => {
                warn!("Failed to get network info: {e}");
                had_error = true;
            }
        }

        // Peer info (aggregated)
        match self.node.get_peer_info() {
            Ok(peers) => {
                let total = peers.0.len();
                let inbound = peers.0.iter().filter(|p| p.inbound).count();
                let outbound = total - inbound;
                let total_sent: u64 = peers.0.iter().map(|p| p.bytes_sent).sum();
                let total_recv: u64 = peers.0.iter().map(|p| p.bytes_received).sum();
                let ping_sum: f64 = peers.0.iter().filter_map(|p| p.ping_time).sum();
                let ping_count = peers.0.iter().filter(|p| p.ping_time.is_some()).count();
                let avg_ping = if ping_count > 0 { ping_sum / ping_count as f64 } else { 0.0 };

                self.metrics.peer_count.set(total as f64);
                self.metrics.peers_inbound.set(inbound as f64);
                self.metrics.peers_outbound.set(outbound as f64);
                self.metrics.peers_total_bytes_sent.set(total_sent as f64);
                self.metrics.peers_total_bytes_received.set(total_recv as f64);
                self.metrics.peers_avg_ping_seconds.set(avg_ping);
                info!("Updated peer info: peers={} (in={}, out={})", total, inbound, outbound);
            }
            Err(e) => {
                warn!("Failed to get peer info: {e}");
                had_error = true;
            }
        }

        // Mining info
        match self.node.get_mining_info() {
            Ok(info) => {
                self.metrics.network_hash_ps.set(info.network_hash_ps);
                self.metrics.mining_pooled_tx.set(info.pooled_tx as f64);
                info!("Updated mining info: hashps={}, pooledtx={}", info.network_hash_ps, info.pooled_tx);
            }
            Err(e) => {
                warn!("Failed to get mining info: {e}");
                had_error = true;
            }
        }

        // Chain tx stats
        match self.node.get_chain_tx_stats() {
            Ok(info) => {
                self.metrics.chain_tx_count.set(info.tx_count as f64);
                if let Some(rate) = info.tx_rate {
                    self.metrics.chain_tx_rate.set(rate);
                }
                self.metrics.chain_tx_window_block_count.set(info.window_block_count as f64);
                if let Some(count) = info.window_tx_count {
                    self.metrics.chain_tx_window_tx_count.set(count as f64);
                }
                if let Some(interval) = info.window_interval {
                    self.metrics.chain_tx_window_interval.set(interval as f64);
                }
                info!("Updated chain tx stats: total_txs={}, rate={:?}", info.tx_count, info.tx_rate);
            }
            Err(e) => {
                warn!("Failed to get chain tx stats: {e}");
                had_error = true;
            }
        }

        // Net totals
        match self.node.get_net_totals() {
            Ok(info) => {
                self.metrics.net_total_bytes_received.set(info.total_bytes_received as f64);
                self.metrics.net_total_bytes_sent.set(info.total_bytes_sent as f64);
                info!("Updated net totals: recv={}, sent={}", info.total_bytes_received, info.total_bytes_sent);
            }
            Err(e) => {
                warn!("Failed to get net totals: {e}");
                had_error = true;
            }
        }

        // Fee estimation at various confirmation targets
        for (target, gauge) in [
            (2, &self.metrics.fee_estimate_2_blocks),
            (6, &self.metrics.fee_estimate_6_blocks),
            (12, &self.metrics.fee_estimate_12_blocks),
            (144, &self.metrics.fee_estimate_144_blocks),
        ] {
            match self.node.estimate_smart_fee(target) {
                Ok(est) => {
                    if let Some(rate) = est.fee_rate {
                        gauge.set(rate);
                    }
                }
                Err(e) => {
                    warn!("Failed to estimate smart fee for {target} blocks: {e}");
                    had_error = true;
                }
            }
        }
        info!("Updated fee estimates");

        // Chain tips
        match self.node.get_chain_tips() {
            Ok(tips) => {
                self.metrics.chain_tips_count.set(tips.0.len() as f64);
                info!("Updated chain tips: count={}", tips.0.len());
            }
            Err(e) => {
                warn!("Failed to get chain tips: {e}");
                had_error = true;
            }
        }

        // Uptime
        match self.node.uptime() {
            Ok(seconds) => {
                self.metrics.node_uptime_seconds.set(seconds as f64);
                info!("Updated uptime: {}s", seconds);
            }
            Err(e) => {
                warn!("Failed to get uptime: {e}");
                had_error = true;
            }
        }

        // Latest block stats (requires block height from blockchain info)
        if let Some(height) = block_height {
            match self.node.get_block_stats_by_height(height as u32) {
                Ok(stats) => {
                    self.metrics.latest_block_txs.set(stats.txs as f64);
                    self.metrics.latest_block_size.set(stats.total_size as f64);
                    self.metrics.latest_block_weight.set(stats.total_weight as f64);
                    self.metrics.latest_block_avg_fee.set(stats.average_fee as f64);
                    self.metrics.latest_block_avg_fee_rate.set(stats.average_fee_rate as f64);
                    self.metrics.latest_block_median_fee.set(stats.median_fee as f64);
                    self.metrics.latest_block_min_fee.set(stats.minimum_fee as f64);
                    self.metrics.latest_block_max_fee.set(stats.max_fee as f64);
                    self.metrics.latest_block_min_fee_rate.set(stats.minimum_fee_rate as f64);
                    self.metrics.latest_block_max_fee_rate.set(stats.max_fee_rate as f64);
                    self.metrics.latest_block_total_fee.set(stats.total_fee as f64);
                    self.metrics.latest_block_subsidy.set(stats.subsidy as f64);
                    self.metrics.latest_block_inputs.set(stats.inputs as f64);
                    self.metrics.latest_block_outputs.set(stats.outputs as f64);
                    self.metrics.latest_block_segwit_txs.set(stats.segwit_txs as f64);
                    self.metrics.latest_block_segwit_total_size.set(stats.segwit_total_size as f64);
                    self.metrics.latest_block_segwit_total_weight.set(stats.segwit_total_weight as f64);
                    self.metrics.latest_block_total_out.set(stats.total_out as f64);
                    self.metrics.latest_block_utxo_increase.set(stats.utxo_increase as f64);
                    self.metrics.latest_block_fee_rate_10th.set(stats.fee_rate_percentiles[0] as f64);
                    self.metrics.latest_block_fee_rate_25th.set(stats.fee_rate_percentiles[1] as f64);
                    self.metrics.latest_block_fee_rate_50th.set(stats.fee_rate_percentiles[2] as f64);
                    self.metrics.latest_block_fee_rate_75th.set(stats.fee_rate_percentiles[3] as f64);
                    self.metrics.latest_block_fee_rate_90th.set(stats.fee_rate_percentiles[4] as f64);
                    info!("Updated latest block stats: height={}, txs={}, total_fee={}", height, stats.txs, stats.total_fee);
                }
                Err(e) => {
                    warn!("Failed to get block stats for height {height}: {e}");
                    had_error = true;
                }
            }
        }

        let duration = start.elapsed().as_secs_f64();
        self.metrics.scrape_duration_seconds.set(duration);
        self.metrics.scrape_error.set(if had_error { 1.0 } else { 0.0 });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::btcnode_metrics_gatherer::Error;
    use crate::btcnode_metrics_gatherer::node::{ChainTxStats, MiningInfo};
    use corepc_client::types::v28::*;

    struct MockNode;

    impl NodeClient for MockNode {
        fn get_blockchain_info(&self) -> Result<GetBlockchainInfo, Error> {
            Ok(GetBlockchainInfo {
                chain: String::from("main"),
                blocks: 800000,
                headers: 800000,
                best_block_hash: String::from(
                    "0000000000000000000000000000000000000000000000000000000000000000"
                ),
                difficulty: 53_911_173_001_054.59,
                time: 1_700_000_000,
                median_time: 1_699_999_000,
                verification_progress: 0.9999,
                initial_block_download: false,
                chain_work: String::new(),
                size_on_disk: 600_000_000_000,
                pruned: false,
                prune_height: None,
                automatic_pruning: None,
                prune_target_size: None,
                softforks: Default::default(),
                warnings: vec![],
            })
        }

        fn get_mempool_info(&self) -> Result<GetMempoolInfo, Error> {
            Ok(GetMempoolInfo {
                loaded: true,
                size: 5000,
                bytes: 3_000_000,
                usage: 10_000_000,
                total_fee: 0.5,
                max_mempool: 300_000_000,
                mempool_min_fee: 0.00001,
                min_relay_tx_fee: 0.00001,
                incremental_relay_fee: 0.00001,
                unbroadcast_count: 3,
                full_rbf: false,
            })
        }

        fn get_network_info(&self) -> Result<GetNetworkInfo, Error> {
            Ok(GetNetworkInfo {
                version: 250000,
                subversion: String::from("/Satoshi:25.0.0/"),
                protocol_version: 70016,
                local_services: String::new(),
                local_services_names: vec![],
                local_relay: true,
                time_offset: -2,
                connections: 125,
                connections_in: 85,
                connections_out: 40,
                network_active: true,
                networks: vec![],
                relay_fee: 0.00001,
                incremental_fee: 0.00001,
                local_addresses: vec![],
                warnings: vec![],
            })
        }

        fn get_peer_info(&self) -> Result<GetPeerInfo, Error> {
            Ok(GetPeerInfo(vec![
                PeerInfo {
                    id: 1,
                    address: "1.2.3.4:8333".into(),
                    address_bind: Some("0.0.0.0:0".into()),
                    address_local: None,
                    network: "ipv4".into(),
                    mapped_as: None,
                    services: "0000000000000409".into(),
                    services_names: vec!["NETWORK".into(), "WITNESS".into()],
                    relay_transactions: true,
                    last_send: 1_700_000_000,
                    last_received: 1_700_000_000,
                    last_transaction: 0,
                    last_block: 0,
                    bytes_sent: 50_000,
                    bytes_received: 100_000,
                    connection_time: 1_699_900_000,
                    time_offset: 0,
                    ping_time: Some(0.05),
                    minimum_ping: Some(0.02),
                    ping_wait: None,
                    version: 70016,
                    subversion: "/Satoshi:25.0.0/".into(),
                    inbound: false,
                    bip152_hb_to: false,
                    bip152_hb_from: false,
                    add_node: None,
                    starting_height: Some(799_990),
                    presynced_headers: Some(-1),
                    ban_score: None,
                    synced_headers: Some(800_000),
                    synced_blocks: Some(800_000),
                    inflight: Some(vec![]),
                    addresses_relay_enabled: None,
                    addresses_processed: None,
                    addresses_rate_limited: None,
                    permissions: vec![],
                    whitelisted: None,
                    minimum_fee_filter: 0.00001,
                    bytes_sent_per_message: Default::default(),
                    bytes_received_per_message: Default::default(),
                    connection_type: Some("outbound-full-relay".into()),
                    transport_protocol_type: "v1".into(),
                    session_id: String::new(),
                },
                PeerInfo {
                    id: 2,
                    address: "5.6.7.8:8333".into(),
                    address_bind: Some("0.0.0.0:0".into()),
                    address_local: None,
                    network: "ipv4".into(),
                    mapped_as: None,
                    services: "0000000000000409".into(),
                    services_names: vec!["NETWORK".into(), "WITNESS".into()],
                    relay_transactions: true,
                    last_send: 1_700_000_000,
                    last_received: 1_700_000_000,
                    last_transaction: 0,
                    last_block: 0,
                    bytes_sent: 30_000,
                    bytes_received: 60_000,
                    connection_time: 1_699_900_000,
                    time_offset: 0,
                    ping_time: Some(0.10),
                    minimum_ping: Some(0.05),
                    ping_wait: None,
                    version: 70016,
                    subversion: "/Satoshi:25.0.0/".into(),
                    inbound: true,
                    bip152_hb_to: false,
                    bip152_hb_from: false,
                    add_node: None,
                    starting_height: Some(799_990),
                    presynced_headers: Some(-1),
                    ban_score: None,
                    synced_headers: Some(800_000),
                    synced_blocks: Some(800_000),
                    inflight: Some(vec![]),
                    addresses_relay_enabled: None,
                    addresses_processed: None,
                    addresses_rate_limited: None,
                    permissions: vec![],
                    whitelisted: None,
                    minimum_fee_filter: 0.00001,
                    bytes_sent_per_message: Default::default(),
                    bytes_received_per_message: Default::default(),
                    connection_type: Some("inbound".into()),
                    transport_protocol_type: "v1".into(),
                    session_id: String::new(),
                },
            ]))
        }

        fn get_mining_info(&self) -> Result<MiningInfo, Error> {
            Ok(MiningInfo {
                blocks: 800_000,
                current_block_weight: Some(3_993_000),
                current_block_tx: Some(2_500),
                difficulty: 53_911_173_001_054.59,
                network_hash_ps: 4.5e17,
                pooled_tx: 5000,
                chain: "main".into(),
                warnings: vec![],
            })
        }

        fn get_chain_tx_stats(&self) -> Result<ChainTxStats, Error> {
            Ok(ChainTxStats {
                time: 1_700_000_000,
                tx_count: 900_000_000,
                window_final_block_hash: "0000000000000000000000000000000000000000000000000000000000000000".into(),
                window_final_block_height: 800_000,
                window_block_count: 4032,
                window_tx_count: Some(12_000_000),
                window_interval: Some(2_419_200),
                tx_rate: Some(4.96),
            })
        }

        fn get_net_totals(&self) -> Result<GetNetTotals, Error> {
            Ok(GetNetTotals {
                total_bytes_received: 5_000_000_000,
                total_bytes_sent: 3_000_000_000,
                time_millis: 1_700_000_000_000,
                upload_target: UploadTarget {
                    timeframe: 86400,
                    target: 0,
                    target_reached: false,
                    serve_historical_blocks: true,
                    bytes_left_in_cycle: 0,
                    time_left_in_cycle: 43200,
                },
            })
        }

        fn estimate_smart_fee(&self, conf_target: u32) -> Result<EstimateSmartFee, Error> {
            let rate = match conf_target {
                2 => 0.00025,
                6 => 0.00015,
                12 => 0.00010,
                144 => 0.00005,
                _ => 0.00010,
            };
            Ok(EstimateSmartFee {
                fee_rate: Some(rate),
                errors: None,
                blocks: conf_target,
            })
        }

        fn get_chain_tips(&self) -> Result<GetChainTips, Error> {
            Ok(GetChainTips(vec![
                ChainTips {
                    height: 800_000,
                    hash: "0000000000000000000000000000000000000000000000000000000000000000".into(),
                    branch_length: 0,
                    status: ChainTipsStatus::Active,
                },
                ChainTips {
                    height: 799_998,
                    hash: "0000000000000000000000000000000000000000000000000000000000000001".into(),
                    branch_length: 2,
                    status: ChainTipsStatus::ValidFork,
                },
            ]))
        }

        fn uptime(&self) -> Result<u32, Error> {
            Ok(86400)
        }

        fn get_block_stats_by_height(&self, _height: u32) -> Result<GetBlockStats, Error> {
            Ok(GetBlockStats {
                average_fee: 15_000,
                average_fee_rate: 25,
                average_tx_size: 500,
                block_hash: "0000000000000000000000000000000000000000000000000000000000000000".into(),
                fee_rate_percentiles: [5, 10, 20, 50, 100],
                height: 800_000,
                inputs: 6000,
                max_fee: 500_000,
                max_fee_rate: 200,
                max_tx_size: 100_000,
                median_fee: 10_000,
                median_time: 1_699_999_000,
                median_tx_size: 250,
                minimum_fee: 500,
                minimum_fee_rate: 1,
                minimum_tx_size: 150,
                outputs: 8000,
                subsidy: 625_000_000,
                segwit_total_size: 1_500_000,
                segwit_total_weight: 3_000_000,
                segwit_txs: 2000,
                time: 1_700_000_000,
                total_out: 500_000_000_000,
                total_size: 2_000_000,
                total_weight: 3_993_000,
                total_fee: 37_500_000,
                txs: 2500,
                utxo_increase: 500,
                utxo_size_increase: 25_000,
                utxo_increase_actual: None,
                utxo_size_increase_actual: None,
            })
        }
    }

    #[test]
    fn test_collect_updates_gauges() {
        let metrics = BitcoinMetrics::new().unwrap();
        let collector = MetricsCollector::new(MockNode, metrics);

        collector.collect();

        // Blockchain info
        assert_eq!(collector.metrics().blocks.get(), 800_000.0);
        assert_eq!(collector.metrics().headers.get(), 800_000.0);
        assert!(collector.metrics().difficulty.get() > 0.0);
        assert_eq!(collector.metrics().initial_block_download.get(), 0.0);
        assert_eq!(collector.metrics().chain_pruned.get(), 0.0);
        assert_eq!(collector.metrics().size_on_disk.get(), 600_000_000_000.0);

        // Mempool info
        assert_eq!(collector.metrics().mempool_transactions.get(), 5000.0);
        assert_eq!(collector.metrics().mempool_bytes.get(), 3_000_000.0);
        assert_eq!(collector.metrics().mempool_total_fee.get(), 0.5);
        assert_eq!(collector.metrics().mempool_unbroadcast_count.get(), 3.0);
        assert_eq!(collector.metrics().mempool_full_rbf.get(), 0.0);

        // Network info
        assert_eq!(collector.metrics().connections.get(), 125.0);
        assert_eq!(collector.metrics().connections_in.get(), 85.0);
        assert_eq!(collector.metrics().connections_out.get(), 40.0);
        assert_eq!(collector.metrics().network_active.get(), 1.0);
        assert_eq!(collector.metrics().protocol_version.get(), 70016.0);
        assert_eq!(collector.metrics().time_offset.get(), -2.0);
        assert_eq!(collector.metrics().relay_fee.get(), 0.00001);
        assert_eq!(collector.metrics().incremental_fee.get(), 0.00001);

        // Peer info
        assert_eq!(collector.metrics().peer_count.get(), 2.0);
        assert_eq!(collector.metrics().peers_inbound.get(), 1.0);
        assert_eq!(collector.metrics().peers_outbound.get(), 1.0);
        assert_eq!(collector.metrics().peers_total_bytes_sent.get(), 80_000.0);
        assert_eq!(collector.metrics().peers_total_bytes_received.get(), 160_000.0);
        assert!((collector.metrics().peers_avg_ping_seconds.get() - 0.075).abs() < 0.001);

        // Mining info
        assert_eq!(collector.metrics().network_hash_ps.get(), 4.5e17);
        assert_eq!(collector.metrics().mining_pooled_tx.get(), 5000.0);

        // Chain tx stats
        assert_eq!(collector.metrics().chain_tx_count.get(), 900_000_000.0);
        assert_eq!(collector.metrics().chain_tx_rate.get(), 4.96);
        assert_eq!(collector.metrics().chain_tx_window_block_count.get(), 4032.0);
        assert_eq!(collector.metrics().chain_tx_window_tx_count.get(), 12_000_000.0);
        assert_eq!(collector.metrics().chain_tx_window_interval.get(), 2_419_200.0);

        // Net totals
        assert_eq!(collector.metrics().net_total_bytes_received.get(), 5_000_000_000.0);
        assert_eq!(collector.metrics().net_total_bytes_sent.get(), 3_000_000_000.0);

        // Fee estimates
        assert_eq!(collector.metrics().fee_estimate_2_blocks.get(), 0.00025);
        assert_eq!(collector.metrics().fee_estimate_6_blocks.get(), 0.00015);
        assert_eq!(collector.metrics().fee_estimate_12_blocks.get(), 0.00010);
        assert_eq!(collector.metrics().fee_estimate_144_blocks.get(), 0.00005);

        // Chain tips
        assert_eq!(collector.metrics().chain_tips_count.get(), 2.0);

        // Uptime
        assert_eq!(collector.metrics().node_uptime_seconds.get(), 86400.0);

        // Latest block stats
        assert_eq!(collector.metrics().latest_block_txs.get(), 2500.0);
        assert_eq!(collector.metrics().latest_block_size.get(), 2_000_000.0);
        assert_eq!(collector.metrics().latest_block_weight.get(), 3_993_000.0);
        assert_eq!(collector.metrics().latest_block_avg_fee.get(), 15_000.0);
        assert_eq!(collector.metrics().latest_block_avg_fee_rate.get(), 25.0);
        assert_eq!(collector.metrics().latest_block_median_fee.get(), 10_000.0);
        assert_eq!(collector.metrics().latest_block_min_fee.get(), 500.0);
        assert_eq!(collector.metrics().latest_block_max_fee.get(), 500_000.0);
        assert_eq!(collector.metrics().latest_block_min_fee_rate.get(), 1.0);
        assert_eq!(collector.metrics().latest_block_max_fee_rate.get(), 200.0);
        assert_eq!(collector.metrics().latest_block_total_fee.get(), 37_500_000.0);
        assert_eq!(collector.metrics().latest_block_subsidy.get(), 625_000_000.0);
        assert_eq!(collector.metrics().latest_block_inputs.get(), 6000.0);
        assert_eq!(collector.metrics().latest_block_outputs.get(), 8000.0);
        assert_eq!(collector.metrics().latest_block_segwit_txs.get(), 2000.0);
        assert_eq!(collector.metrics().latest_block_total_out.get(), 500_000_000_000.0);
        assert_eq!(collector.metrics().latest_block_utxo_increase.get(), 500.0);
        assert_eq!(collector.metrics().latest_block_fee_rate_10th.get(), 5.0);
        assert_eq!(collector.metrics().latest_block_fee_rate_25th.get(), 10.0);
        assert_eq!(collector.metrics().latest_block_fee_rate_50th.get(), 20.0);
        assert_eq!(collector.metrics().latest_block_fee_rate_75th.get(), 50.0);
        assert_eq!(collector.metrics().latest_block_fee_rate_90th.get(), 100.0);

        // Meta
        assert_eq!(collector.metrics().scrape_error.get(), 0.0);
    }

    struct PartialFailNode;

    impl NodeClient for PartialFailNode {
        fn get_blockchain_info(&self) -> Result<GetBlockchainInfo, Error> {
            MockNode.get_blockchain_info()
        }

        fn get_mempool_info(&self) -> Result<GetMempoolInfo, Error> {
            Err(Error::Config("simulated failure".to_string()))
        }

        fn get_network_info(&self) -> Result<GetNetworkInfo, Error> {
            MockNode.get_network_info()
        }

        fn get_peer_info(&self) -> Result<GetPeerInfo, Error> {
            MockNode.get_peer_info()
        }

        fn get_mining_info(&self) -> Result<MiningInfo, Error> {
            MockNode.get_mining_info()
        }

        fn get_chain_tx_stats(&self) -> Result<ChainTxStats, Error> {
            MockNode.get_chain_tx_stats()
        }

        fn get_net_totals(&self) -> Result<GetNetTotals, Error> {
            MockNode.get_net_totals()
        }

        fn estimate_smart_fee(&self, conf_target: u32) -> Result<EstimateSmartFee, Error> {
            MockNode.estimate_smart_fee(conf_target)
        }

        fn get_chain_tips(&self) -> Result<GetChainTips, Error> {
            MockNode.get_chain_tips()
        }

        fn uptime(&self) -> Result<u32, Error> {
            MockNode.uptime()
        }

        fn get_block_stats_by_height(&self, height: u32) -> Result<GetBlockStats, Error> {
            MockNode.get_block_stats_by_height(height)
        }
    }

    #[test]
    fn test_partial_failure_sets_error_gauge() {
        let metrics = BitcoinMetrics::new().unwrap();
        let collector = MetricsCollector::new(PartialFailNode, metrics);

        collector.collect();

        // Blockchain info should still be collected
        assert_eq!(collector.metrics().blocks.get(), 800_000.0);
        // But error gauge should be set
        assert_eq!(collector.metrics().scrape_error.get(), 1.0);
    }
}
