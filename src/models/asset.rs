use super::asset_policy::{AssetInformation, PolicyTransaction};
use crate::utils;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct AccountHoldingAsset {
    pub account: String,
    pub amount: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountsHoldingAsset {
    pub data: Vec<AccountHoldingAsset>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AddressHoldingAsset {
    pub address: String,
    pub amount: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AddressesHoldingAsset {
    pub data: Vec<AddressHoldingAsset>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetInformations {
    pub data: AssetInformation,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetTransactions {
    pub data: Vec<PolicyTransaction>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Metadata {
    pub json: Option<HashMap<String, serde_json::Value>>,
    pub key: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetUpdate {
    pub block_timestamp: i64,
    pub metadata: Metadata,
    pub mint_amount: i64,
    pub tx_hash: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetUpdates {
    pub data: Vec<AssetUpdate>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetUtxo {
    pub address: String,
    pub amount: i64,
    pub index: i64,
    pub slot: i64,
    pub tx_hash: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetUtxos {
    pub data: Vec<AssetUtxo>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}
