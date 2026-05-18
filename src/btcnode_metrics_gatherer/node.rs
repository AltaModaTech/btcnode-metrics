// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 AltaModa Technologies, LLC
// SPDX-FileCopyrightText: Contributors to the btcnode-metrics project.

use corepc_client::client_sync::{v29::Client, Auth};
use corepc_client::types::v29::{
    EstimateSmartFee, GetBlockStats, GetBlockchainInfo, GetChainTips, GetChainTxStats,
    GetMempoolInfo, GetMiningInfo, GetNetTotals, GetNetworkInfo, GetPeerInfo,
};

use super::Error;
use super::config::NodeConfig;

pub trait NodeClient: Send + Sync {
    fn get_blockchain_info(&self) -> Result<GetBlockchainInfo, Error>;
    fn get_mempool_info(&self) -> Result<GetMempoolInfo, Error>;
    fn get_network_info(&self) -> Result<GetNetworkInfo, Error>;
    fn get_peer_info(&self) -> Result<GetPeerInfo, Error>;
    fn get_mining_info(&self) -> Result<GetMiningInfo, Error>;
    fn get_chain_tx_stats(&self) -> Result<GetChainTxStats, Error>;
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

    fn get_mining_info(&self) -> Result<GetMiningInfo, Error> {
        Ok(self.client.get_mining_info()?)
    }

    fn get_chain_tx_stats(&self) -> Result<GetChainTxStats, Error> {
        Ok(self.client.get_chain_tx_stats()?)
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
        Ok(self.client.get_block_stats_by_height(height, None)?)
    }
}
