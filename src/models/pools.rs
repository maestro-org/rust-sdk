use crate::utils;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pool {
    pub pool_id_bech32: String,
    pub ticker: String,
}

#[derive(Deserialize)]
pub struct RegisteredPools {
    pub data: Vec<Pool>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct Block {
    pub abs_slot: i64,
    pub block_hash: String,
    pub block_height: i64,
    pub block_time: i64,
    pub epoch_no: i64,
    pub epoch_slot: i64,
}

#[derive(Deserialize)]
pub struct PoolMintedBlocks {
    pub data: Vec<Block>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct StakePoolDelegator {
    pub active_epoch_no: i64,
    pub amount: String,
    pub latest_delegation_tx_hash: String,
    pub stake_address: String,
}

#[derive(Deserialize)]
pub struct StakePoolDelegators {
    pub data: Vec<StakePoolDelegator>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct StakePoolHistoryData {
    pub active_stake: i64,
    pub active_stake_pct: Option<String>,
    pub block_cnt: i64,
    pub deleg_rewards: i64,
    pub delegator_cnt: i64,
    pub epoch_no: i64,
    pub epoch_ros: String,
    pub fixed_cost: i64,
    pub margin: i64,
    pub pool_fees: i64,
    pub saturation_pct: serde_json::Value,
}

#[derive(Deserialize)]
pub struct StakePoolHistory {
    pub data: Vec<StakePoolHistoryData>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct Relay {
    pub dns: String,
    pub ipv4: String,
    pub ipv6: String,
    pub port: i64,
    pub srv: String,
}

#[derive(Deserialize)]
pub struct StakePoolDetails {
    pub active_epoch_no: i64,
    pub active_stake: i64,
    pub block_count: i64,
    pub fixed_cost: i64,
    pub live_delegators: i64,
    pub live_pledge: i64,
    pub live_saturation: String,
    pub live_stake: i64,
    pub margin: i64,
    pub meta_hash: serde_json::Value,
    pub meta_json: serde_json::Value,
    pub meta_url: serde_json::Value,
    pub op_cert: String,
    pub op_cert_counter: i64,
    pub owners: Vec<String>,
    pub pledge: i64,
    pub pool_id_bech32: String,
    pub pool_id_hex: String,
    pub pool_status: String,
    pub relays: Vec<Relay>,
    pub retiring_epoch: serde_json::Value,
    pub reward_addr: String,
    pub sigma: String,
    pub vrf_key_hash: String,
}

#[derive(Deserialize)]
pub struct StakePoolInformation {
    pub data: StakePoolDetails,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize)]
pub struct Poolmetadata {
    pub meta_hash: String,
    pub meta_json: serde_json::Value,
    pub meta_url: String,
    pub pool_id_bech32: String,
}

#[derive(Deserialize)]
pub struct StakePoolMetadata {
    pub data: Poolmetadata,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize)]
pub struct RelaysAndId {
    pub pool_id_bech32: String,
    pub relays: Vec<Relay>,
}

#[derive(Deserialize)]
pub struct StakePoolRelays {
    pub data: Vec<RelaysAndId>,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize)]
pub struct StakePoolUpdates {
    pub data: Vec<StakePoolDetails>,
    pub last_updated: utils::LastUpdated,
}
