use crate::utils;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChainTipData {
    pub block_hash: String,
    pub height: i64,
    pub slot: i64,
}

#[derive(Deserialize)]
pub struct ChainTip {
    pub data: ChainTipData,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize)]
pub struct EraTimeStamp {
    pub epoch: i64,
    pub slot: i64,
    pub time: i64,
}

#[derive(Deserialize)]
pub struct EraParams {
    pub epoch_length: i64,
    pub safe_zone: i64,
    pub slot_length: i64,
}

#[derive(Deserialize)]
pub struct Era {
    pub end: EraTimeStamp,
    pub parameters: EraParams,
    pub start: EraTimeStamp,
}

#[derive(Deserialize)]
pub struct EraHistory {
    pub data: Vec<Era>,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize)]
pub struct ProtocolParameters {
    pub data: ProtocolParams,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize)]
pub struct ExUnits {
    pub memory: i64,
    pub steps: i64,
}

#[derive(Deserialize)]
pub struct StringExUnits {
    pub memory: String,
    pub steps: String,
}

#[derive(Deserialize)]
pub struct ProtocolVersion {
    pub major: i64,
    pub minor: i64,
}

#[derive(Deserialize)]
pub struct ProtocolParams {
    pub coins_per_utxo_byte: i64,
    pub collateral_percentage: i64,
    pub cost_models: serde_json::Value,
    pub desired_number_of_pools: i64,
    pub max_block_body_size: i64,
    pub max_block_header_size: i64,
    pub max_collateral_inputs: i64,
    pub max_execution_units_per_block: ExUnits,
    pub max_execution_units_per_transaction: ExUnits,
    pub max_tx_size: i64,
    pub max_value_size: i64,
    pub min_fee_coefficient: i64,
    pub min_fee_constant: i64,
    pub min_pool_cost: i64,
    pub monetary_expansion: String,
    pub pool_deposit: i64,
    pub pool_influence: String,
    pub pool_retirement_epoch_bound: i64,
    pub prices: StringExUnits,
    pub protocol_version: ProtocolVersion,
    pub stake_key_deposit: i64,
    pub treasury_expansion: String,
}
