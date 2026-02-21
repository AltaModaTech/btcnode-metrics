use prometheus::{Gauge, Registry, Opts};

use super::Error;

pub struct BitcoinMetrics {
    pub registry: Registry,

    // Blockchain info
    pub blocks: Gauge,
    pub headers: Gauge,
    pub difficulty: Gauge,
    pub verification_progress: Gauge,
    pub size_on_disk: Gauge,
    pub initial_block_download: Gauge,
    pub chain_pruned: Gauge,

    // Mempool info
    pub mempool_transactions: Gauge,
    pub mempool_bytes: Gauge,
    pub mempool_usage: Gauge,
    pub mempool_max_bytes: Gauge,
    pub mempool_min_fee: Gauge,
    pub mempool_total_fee: Gauge,
    pub mempool_min_relay_tx_fee: Gauge,
    pub mempool_incremental_relay_fee: Gauge,
    pub mempool_unbroadcast_count: Gauge,
    pub mempool_full_rbf: Gauge,

    // Network info
    pub connections: Gauge,
    pub connections_in: Gauge,
    pub connections_out: Gauge,
    pub network_active: Gauge,
    pub node_version: Gauge,
    pub protocol_version: Gauge,
    pub time_offset: Gauge,
    pub relay_fee: Gauge,
    pub incremental_fee: Gauge,

    // Peer info (aggregated)
    pub peer_count: Gauge,
    pub peers_inbound: Gauge,
    pub peers_outbound: Gauge,
    pub peers_total_bytes_sent: Gauge,
    pub peers_total_bytes_received: Gauge,
    pub peers_avg_ping_seconds: Gauge,

    // Mining info
    pub network_hash_ps: Gauge,
    pub mining_pooled_tx: Gauge,

    // Chain tx stats
    pub chain_tx_count: Gauge,
    pub chain_tx_rate: Gauge,
    pub chain_tx_window_block_count: Gauge,
    pub chain_tx_window_tx_count: Gauge,
    pub chain_tx_window_interval: Gauge,

    // Net totals
    pub net_total_bytes_received: Gauge,
    pub net_total_bytes_sent: Gauge,

    // Fee estimation (BTC/kvB for various confirmation targets)
    pub fee_estimate_2_blocks: Gauge,
    pub fee_estimate_6_blocks: Gauge,
    pub fee_estimate_12_blocks: Gauge,
    pub fee_estimate_144_blocks: Gauge,

    // Chain tips
    pub chain_tips_count: Gauge,

    // Uptime
    pub node_uptime_seconds: Gauge,

    // Latest block stats
    pub latest_block_txs: Gauge,
    pub latest_block_size: Gauge,
    pub latest_block_weight: Gauge,
    pub latest_block_avg_fee: Gauge,
    pub latest_block_avg_fee_rate: Gauge,
    pub latest_block_median_fee: Gauge,
    pub latest_block_min_fee: Gauge,
    pub latest_block_max_fee: Gauge,
    pub latest_block_min_fee_rate: Gauge,
    pub latest_block_max_fee_rate: Gauge,
    pub latest_block_total_fee: Gauge,
    pub latest_block_subsidy: Gauge,
    pub latest_block_inputs: Gauge,
    pub latest_block_outputs: Gauge,
    pub latest_block_segwit_txs: Gauge,
    pub latest_block_segwit_total_size: Gauge,
    pub latest_block_segwit_total_weight: Gauge,
    pub latest_block_total_out: Gauge,
    pub latest_block_utxo_increase: Gauge,
    pub latest_block_fee_rate_10th: Gauge,
    pub latest_block_fee_rate_25th: Gauge,
    pub latest_block_fee_rate_50th: Gauge,
    pub latest_block_fee_rate_75th: Gauge,
    pub latest_block_fee_rate_90th: Gauge,

    // Collector meta
    pub scrape_duration_seconds: Gauge,
    pub scrape_error: Gauge,
}

macro_rules! register_gauge {
    ($registry:expr, $name:expr, $help:expr) => {{
        let gauge = Gauge::with_opts(Opts::new($name, $help))?;
        $registry.register(Box::new(gauge.clone()))?;
        gauge
    }};
}

impl BitcoinMetrics {
    pub fn new() -> Result<Self, Error> {
        let registry = Registry::new();

        // Blockchain info
        let blocks = register_gauge!(registry, "bitcoin_blocks", "Current block height");
        let headers = register_gauge!(registry, "bitcoin_headers", "Current number of headers");
        let difficulty = register_gauge!(registry, "bitcoin_difficulty", "Current mining difficulty");
        let verification_progress = register_gauge!(registry, "bitcoin_verification_progress", "Estimate of verification progress [0..1]");
        let size_on_disk = register_gauge!(registry, "bitcoin_size_on_disk_bytes", "Estimated size of the block and undo files on disk");
        let initial_block_download = register_gauge!(registry, "bitcoin_initial_block_download", "Whether node is in initial block download (1=true, 0=false)");
        let chain_pruned = register_gauge!(registry, "bitcoin_chain_pruned", "Whether the blockchain is pruned (1=true, 0=false)");

        // Mempool info
        let mempool_transactions = register_gauge!(registry, "bitcoin_mempool_transactions", "Current number of transactions in the mempool");
        let mempool_bytes = register_gauge!(registry, "bitcoin_mempool_bytes", "Sum of all virtual transaction sizes in the mempool");
        let mempool_usage = register_gauge!(registry, "bitcoin_mempool_usage_bytes", "Total memory usage for the mempool");
        let mempool_max_bytes = register_gauge!(registry, "bitcoin_mempool_max_bytes", "Maximum memory usage for the mempool");
        let mempool_min_fee = register_gauge!(registry, "bitcoin_mempool_min_fee_btc_per_kvb", "Minimum fee rate in BTC/kvB for tx to be accepted");
        let mempool_total_fee = register_gauge!(registry, "bitcoin_mempool_total_fee_btc", "Total fees of all transactions in the mempool in BTC");
        let mempool_min_relay_tx_fee = register_gauge!(registry, "bitcoin_mempool_min_relay_tx_fee_btc_per_kvb", "Minimum relay transaction fee in BTC/kvB");
        let mempool_incremental_relay_fee = register_gauge!(registry, "bitcoin_mempool_incremental_relay_fee_btc_per_kvb", "Minimum fee rate increment for mempool limiting or BIP 125 replacement in BTC/kvB");
        let mempool_unbroadcast_count = register_gauge!(registry, "bitcoin_mempool_unbroadcast_count", "Number of transactions that haven't been broadcast yet");
        let mempool_full_rbf = register_gauge!(registry, "bitcoin_mempool_full_rbf", "Whether full replace-by-fee is enabled (1=true, 0=false)");

        // Network info
        let connections = register_gauge!(registry, "bitcoin_connections", "Total number of connections");
        let connections_in = register_gauge!(registry, "bitcoin_connections_in", "Number of inbound connections");
        let connections_out = register_gauge!(registry, "bitcoin_connections_out", "Number of outbound connections");
        let network_active = register_gauge!(registry, "bitcoin_network_active", "Whether p2p networking is active (1=true, 0=false)");
        let node_version = register_gauge!(registry, "bitcoin_version", "Bitcoin node version as integer");
        let protocol_version = register_gauge!(registry, "bitcoin_protocol_version", "Protocol version number");
        let time_offset = register_gauge!(registry, "bitcoin_time_offset_seconds", "Time offset from network median in seconds");
        let relay_fee = register_gauge!(registry, "bitcoin_relay_fee_btc_per_kvb", "Minimum relay fee for transactions in BTC/kvB");
        let incremental_fee = register_gauge!(registry, "bitcoin_incremental_fee_btc_per_kvb", "Minimum fee increment for mempool limiting in BTC/kvB");

        // Peer info (aggregated)
        let peer_count = register_gauge!(registry, "bitcoin_peer_count", "Number of connected peers");
        let peers_inbound = register_gauge!(registry, "bitcoin_peers_inbound", "Number of inbound peers");
        let peers_outbound = register_gauge!(registry, "bitcoin_peers_outbound", "Number of outbound peers");
        let peers_total_bytes_sent = register_gauge!(registry, "bitcoin_peers_total_bytes_sent", "Total bytes sent across all peers");
        let peers_total_bytes_received = register_gauge!(registry, "bitcoin_peers_total_bytes_received", "Total bytes received across all peers");
        let peers_avg_ping_seconds = register_gauge!(registry, "bitcoin_peers_avg_ping_seconds", "Average ping time across all peers in seconds");

        // Mining info
        let network_hash_ps = register_gauge!(registry, "bitcoin_network_hash_per_second", "Estimated network hashes per second");
        let mining_pooled_tx = register_gauge!(registry, "bitcoin_mining_pooled_transactions", "Number of transactions in the mining pool");

        // Chain tx stats
        let chain_tx_count = register_gauge!(registry, "bitcoin_chain_tx_count", "Total number of transactions in the chain");
        let chain_tx_rate = register_gauge!(registry, "bitcoin_chain_tx_rate_per_second", "Average transaction rate per second over the window");
        let chain_tx_window_block_count = register_gauge!(registry, "bitcoin_chain_tx_window_block_count", "Number of blocks in the stats window");
        let chain_tx_window_tx_count = register_gauge!(registry, "bitcoin_chain_tx_window_tx_count", "Number of transactions in the stats window");
        let chain_tx_window_interval = register_gauge!(registry, "bitcoin_chain_tx_window_interval_seconds", "Elapsed time of the stats window in seconds");

        // Net totals
        let net_total_bytes_received = register_gauge!(registry, "bitcoin_net_total_bytes_received", "Total bytes received since node start");
        let net_total_bytes_sent = register_gauge!(registry, "bitcoin_net_total_bytes_sent", "Total bytes sent since node start");

        // Fee estimation
        let fee_estimate_2_blocks = register_gauge!(registry, "bitcoin_fee_estimate_2_blocks_btc_per_kvb", "Estimated fee rate for confirmation within 2 blocks in BTC/kvB");
        let fee_estimate_6_blocks = register_gauge!(registry, "bitcoin_fee_estimate_6_blocks_btc_per_kvb", "Estimated fee rate for confirmation within 6 blocks in BTC/kvB");
        let fee_estimate_12_blocks = register_gauge!(registry, "bitcoin_fee_estimate_12_blocks_btc_per_kvb", "Estimated fee rate for confirmation within 12 blocks in BTC/kvB");
        let fee_estimate_144_blocks = register_gauge!(registry, "bitcoin_fee_estimate_144_blocks_btc_per_kvb", "Estimated fee rate for confirmation within 144 blocks in BTC/kvB");

        // Chain tips
        let chain_tips_count = register_gauge!(registry, "bitcoin_chain_tips_count", "Number of known chain tips (forks)");

        // Uptime
        let node_uptime_seconds = register_gauge!(registry, "bitcoin_node_uptime_seconds", "Node uptime in seconds");

        // Latest block stats
        let latest_block_txs = register_gauge!(registry, "bitcoin_latest_block_transactions", "Number of transactions in the latest block");
        let latest_block_size = register_gauge!(registry, "bitcoin_latest_block_size_bytes", "Total size of the latest block in bytes");
        let latest_block_weight = register_gauge!(registry, "bitcoin_latest_block_weight", "Total weight of the latest block");
        let latest_block_avg_fee = register_gauge!(registry, "bitcoin_latest_block_avg_fee_sat", "Average fee per transaction in the latest block in satoshis");
        let latest_block_avg_fee_rate = register_gauge!(registry, "bitcoin_latest_block_avg_fee_rate_sat_per_vb", "Average fee rate in the latest block in sat/vB");
        let latest_block_median_fee = register_gauge!(registry, "bitcoin_latest_block_median_fee_sat", "Median fee in the latest block in satoshis");
        let latest_block_min_fee = register_gauge!(registry, "bitcoin_latest_block_min_fee_sat", "Minimum fee in the latest block in satoshis");
        let latest_block_max_fee = register_gauge!(registry, "bitcoin_latest_block_max_fee_sat", "Maximum fee in the latest block in satoshis");
        let latest_block_min_fee_rate = register_gauge!(registry, "bitcoin_latest_block_min_fee_rate_sat_per_vb", "Minimum fee rate in the latest block in sat/vB");
        let latest_block_max_fee_rate = register_gauge!(registry, "bitcoin_latest_block_max_fee_rate_sat_per_vb", "Maximum fee rate in the latest block in sat/vB");
        let latest_block_total_fee = register_gauge!(registry, "bitcoin_latest_block_total_fee_sat", "Total fees in the latest block in satoshis");
        let latest_block_subsidy = register_gauge!(registry, "bitcoin_latest_block_subsidy_sat", "Block subsidy (reward) of the latest block in satoshis");
        let latest_block_inputs = register_gauge!(registry, "bitcoin_latest_block_inputs", "Number of inputs in the latest block (excluding coinbase)");
        let latest_block_outputs = register_gauge!(registry, "bitcoin_latest_block_outputs", "Number of outputs in the latest block");
        let latest_block_segwit_txs = register_gauge!(registry, "bitcoin_latest_block_segwit_transactions", "Number of segwit transactions in the latest block");
        let latest_block_segwit_total_size = register_gauge!(registry, "bitcoin_latest_block_segwit_total_size_bytes", "Total size of segwit transactions in the latest block");
        let latest_block_segwit_total_weight = register_gauge!(registry, "bitcoin_latest_block_segwit_total_weight", "Total weight of segwit transactions in the latest block");
        let latest_block_total_out = register_gauge!(registry, "bitcoin_latest_block_total_out_sat", "Total output value in the latest block in satoshis (excluding coinbase)");
        let latest_block_utxo_increase = register_gauge!(registry, "bitcoin_latest_block_utxo_increase", "Change in UTXO count from the latest block");
        let latest_block_fee_rate_10th = register_gauge!(registry, "bitcoin_latest_block_fee_rate_10th_percentile_sat_per_vb", "10th percentile fee rate in the latest block in sat/vB");
        let latest_block_fee_rate_25th = register_gauge!(registry, "bitcoin_latest_block_fee_rate_25th_percentile_sat_per_vb", "25th percentile fee rate in the latest block in sat/vB");
        let latest_block_fee_rate_50th = register_gauge!(registry, "bitcoin_latest_block_fee_rate_50th_percentile_sat_per_vb", "50th percentile (median) fee rate in the latest block in sat/vB");
        let latest_block_fee_rate_75th = register_gauge!(registry, "bitcoin_latest_block_fee_rate_75th_percentile_sat_per_vb", "75th percentile fee rate in the latest block in sat/vB");
        let latest_block_fee_rate_90th = register_gauge!(registry, "bitcoin_latest_block_fee_rate_90th_percentile_sat_per_vb", "90th percentile fee rate in the latest block in sat/vB");

        // Collector meta
        let scrape_duration_seconds = register_gauge!(registry, "bitcoin_collector_last_scrape_duration_seconds", "Duration of the last metrics collection in seconds");
        let scrape_error = register_gauge!(registry, "bitcoin_collector_last_scrape_error", "Whether the last scrape had an error (1=error, 0=ok)");

        Ok(Self {
            registry,
            blocks,
            headers,
            difficulty,
            verification_progress,
            size_on_disk,
            initial_block_download,
            chain_pruned,
            mempool_transactions,
            mempool_bytes,
            mempool_usage,
            mempool_max_bytes,
            mempool_min_fee,
            mempool_total_fee,
            mempool_min_relay_tx_fee,
            mempool_incremental_relay_fee,
            mempool_unbroadcast_count,
            mempool_full_rbf,
            connections,
            connections_in,
            connections_out,
            network_active,
            node_version,
            protocol_version,
            time_offset,
            relay_fee,
            incremental_fee,
            peer_count,
            peers_inbound,
            peers_outbound,
            peers_total_bytes_sent,
            peers_total_bytes_received,
            peers_avg_ping_seconds,
            network_hash_ps,
            mining_pooled_tx,
            chain_tx_count,
            chain_tx_rate,
            chain_tx_window_block_count,
            chain_tx_window_tx_count,
            chain_tx_window_interval,
            net_total_bytes_received,
            net_total_bytes_sent,
            fee_estimate_2_blocks,
            fee_estimate_6_blocks,
            fee_estimate_12_blocks,
            fee_estimate_144_blocks,
            chain_tips_count,
            node_uptime_seconds,
            latest_block_txs,
            latest_block_size,
            latest_block_weight,
            latest_block_avg_fee,
            latest_block_avg_fee_rate,
            latest_block_median_fee,
            latest_block_min_fee,
            latest_block_max_fee,
            latest_block_min_fee_rate,
            latest_block_max_fee_rate,
            latest_block_total_fee,
            latest_block_subsidy,
            latest_block_inputs,
            latest_block_outputs,
            latest_block_segwit_txs,
            latest_block_segwit_total_size,
            latest_block_segwit_total_weight,
            latest_block_total_out,
            latest_block_utxo_increase,
            latest_block_fee_rate_10th,
            latest_block_fee_rate_25th,
            latest_block_fee_rate_50th,
            latest_block_fee_rate_75th,
            latest_block_fee_rate_90th,
            scrape_duration_seconds,
            scrape_error,
        })
    }
}
