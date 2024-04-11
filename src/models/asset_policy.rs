use super::addresses::Utxo;
use crate::utils;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct AssetNameAndAmount {
    pub name: String,
    pub amount: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountHolding {
    pub account: String,
    pub assets: Vec<AssetNameAndAmount>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountsHoldingPolicy {
    pub data: Vec<AccountHolding>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AddressHolding {
    pub address: String,
    pub assets: Vec<AssetNameAndAmount>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AddressesHoldingPolicy {
    pub data: Vec<AddressHolding>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Standards {
    pub cip25_metadata: HashMap<String, serde_json::Value>,
    pub cip68_metadata: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetInformation {
    pub asset_name: String,
    pub asset_name_ascii: String,
    pub asset_standards: Standards,
    pub burn_tx_count: i64,
    pub fingerprint: String,
    pub first_mint_time: i64,
    pub first_mint_tx: String,
    pub latest_mint_tx_metadata: HashMap<String, serde_json::Value>,
    pub mint_tx_count: i64,
    pub token_registry_metadata: HashMap<String, serde_json::Value>,
    pub total_supply: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PolicyInformation {
    pub data: Vec<AssetInformation>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PolicyTransaction {
    pub block_height: i64,
    pub epoch_no: i64,
    pub tx_hash: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PolicyTransactions {
    pub data: Vec<PolicyTransaction>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UtxosContainingPolicy {
    pub data: Vec<Utxo>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}
