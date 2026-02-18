use corepc_client::client_sync::{v28::Client, Auth};
use corepc_client::types::v28::{
    EstimateSmartFee, GetBlockStats, GetBlockchainInfo, GetChainTips, GetMempoolInfo, GetNetTotals,
    GetNetworkInfo, GetPeerInfo,
};
use serde::Deserialize;

use super::Error;
use super::config::NodeConfig;

/// Custom type for `getmininginfo` that fixes `network_hash_ps` from `i64` to `f64`.
///
/// Bitcoin Core returns `networkhashps` as a floating-point number (e.g. `1.02e+21`)
/// but the upstream `corepc-types` crate incorrectly declares the field as `i64`,
/// which causes deserialization to fail on mainnet.
#[derive(Clone, Debug, Deserialize)]
pub struct MiningInfo {
    pub blocks: u64,
    #[serde(rename = "currentblockweight")]
    pub current_block_weight: Option<u64>,
    #[serde(rename = "currentblocktx")]
    pub current_block_tx: Option<i64>,
    pub difficulty: f64,
    #[serde(rename = "networkhashps")]
    pub network_hash_ps: f64,
    #[serde(rename = "pooledtx")]
    pub pooled_tx: i64,
    pub chain: String,
    pub warnings: Vec<String>,
}

/// Custom type for `getchaintxstats` that fixes `tx_rate` from `Option<i64>` to `Option<f64>`.
///
/// Bitcoin Core returns `txrate` as a floating-point number (e.g. `4.56`)
/// but the upstream `corepc-types` crate incorrectly declares the field as `Option<i64>`,
/// which causes deserialization to fail.
#[derive(Clone, Debug, Deserialize)]
pub struct ChainTxStats {
    pub time: i64,
    #[serde(rename = "txcount")]
    pub tx_count: i64,
    pub window_final_block_hash: String,
    pub window_final_block_height: i64,
    pub window_block_count: i64,
    pub window_tx_count: Option<i64>,
    pub window_interval: Option<i64>,
    #[serde(rename = "txrate")]
    pub tx_rate: Option<f64>,
}

pub trait NodeClient: Send + Sync {
    fn get_blockchain_info(&self) -> Result<GetBlockchainInfo, Error>;
    fn get_mempool_info(&self) -> Result<GetMempoolInfo, Error>;
    fn get_network_info(&self) -> Result<GetNetworkInfo, Error>;
    fn get_peer_info(&self) -> Result<GetPeerInfo, Error>;
    fn get_mining_info(&self) -> Result<MiningInfo, Error>;
    fn get_chain_tx_stats(&self) -> Result<ChainTxStats, Error>;
    fn get_net_totals(&self) -> Result<GetNetTotals, Error>;
    fn estimate_smart_fee(&self, conf_target: u32) -> Result<EstimateSmartFee, Error>;
    fn get_chain_tips(&self) -> Result<GetChainTips, Error>;
    fn uptime(&self) -> Result<u32, Error>;
    fn get_block_stats_by_height(&self, height: u32) -> Result<GetBlockStats, Error>;
}

pub struct BitcoinNode {
    client: Client,
}

impl BitcoinNode {
    pub fn new(config: &NodeConfig) -> Result<Self, Error> {
        let auth = Auth::UserPass(config.rpc_user.clone(), config.rpc_password.clone());
        let client = Client::new_with_auth(&config.rpc_url, auth)
            .map_err(|e| Error::Config(format!("failed to create RPC client: {e}")))?;
        Ok(Self { client })
    }
}

impl NodeClient for BitcoinNode {
    fn get_blockchain_info(&self) -> Result<GetBlockchainInfo, Error> {
        Ok(self.client.get_blockchain_info()?)
    }

    fn get_mempool_info(&self) -> Result<GetMempoolInfo, Error> {
        Ok(self.client.get_mempool_info()?)
    }

    fn get_network_info(&self) -> Result<GetNetworkInfo, Error> {
        Ok(self.client.get_network_info()?)
    }

    fn get_peer_info(&self) -> Result<GetPeerInfo, Error> {
        Ok(self.client.get_peer_info()?)
    }

    fn get_mining_info(&self) -> Result<MiningInfo, Error> {
        // Bypass upstream GetMiningInfo (which declares network_hash_ps as i64)
        // and deserialize directly into our corrected MiningInfo type.
        Ok(self.client.call::<MiningInfo>("getmininginfo", &[])?)
    }

    fn get_chain_tx_stats(&self) -> Result<ChainTxStats, Error> {
        // Bypass upstream GetChainTxStats (which declares tx_rate as Option<i64>)
        // and deserialize directly into our corrected ChainTxStats type.
        Ok(self.client.call::<ChainTxStats>("getchaintxstats", &[])?)
    }

    fn get_net_totals(&self) -> Result<GetNetTotals, Error> {
        Ok(self.client.get_net_totals()?)
    }

    fn estimate_smart_fee(&self, conf_target: u32) -> Result<EstimateSmartFee, Error> {
        Ok(self.client.estimate_smart_fee(conf_target)?)
    }

    fn get_chain_tips(&self) -> Result<GetChainTips, Error> {
        Ok(self.client.get_chain_tips()?)
    }

    fn uptime(&self) -> Result<u32, Error> {
        Ok(self.client.uptime()?)
    }

    fn get_block_stats_by_height(&self, height: u32) -> Result<GetBlockStats, Error> {
        Ok(self.client.get_block_stats_by_height(height)?)
    }
}
