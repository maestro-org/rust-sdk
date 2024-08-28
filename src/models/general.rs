use crate::utils;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ChainTipData {
    pub block_hash: String,
    pub height: i64,
    pub slot: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChainTip {
    pub data: ChainTipData,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EraTimeStamp {
    pub epoch: i64,
    pub slot: i64,
    pub time: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EraParams {
    pub epoch_length: i64,
    pub safe_zone: i64,
    pub slot_length: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Era {
    pub end: EraTimeStamp,
    pub parameters: EraParams,
    pub start: EraTimeStamp,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EraSummaries {
    pub data: Vec<Era>,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProtocolParameters {
    pub data: ProtocolParametersData,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Bytes {
    pub bytes: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExUnits {
    pub memory: u64,
    pub cpu: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LovelaceAmount {
    pub lovelace: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProtocolVersion {
    pub major: u64,
    pub minor: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlutusCostModels {
    pub plutus_v1: Vec<u64>,
    pub plutus_v2: Vec<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ScriptExecutionPrices {
    pub memory: String,
    pub cpu: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProtocolParametersData {
    pub collateral_percentage: u64,
    pub desired_number_of_stake_pools: u64,
    pub max_block_body_size: Bytes,
    pub max_block_header_size: Bytes,
    pub max_collateral_inputs: u64,
    pub max_execution_units_per_block: ExUnits,
    pub max_execution_units_per_transaction: ExUnits,
    pub max_transaction_size: Bytes,
    pub max_value_size: Bytes,
    pub min_fee_coefficient: u64,
    pub min_fee_constant: LovelaceAmount,
    pub min_stake_pool_cost: LovelaceAmount,
    pub min_utxo_deposit_coefficient: u64,
    pub min_utxo_deposit_constant: LovelaceAmount,
    pub monetary_expansion: String,
    pub plutus_cost_models: PlutusCostModels,
    pub script_execution_prices: ScriptExecutionPrices,
    pub stake_credential_deposit: LovelaceAmount,
    pub stake_pool_deposit: LovelaceAmount,
    pub stake_pool_pledge_influence: String,
    pub stake_pool_retirement_epoch_bound: u64,
    pub treasury_expansion: String,
    pub version: ProtocolVersion,
}
