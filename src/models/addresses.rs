use crate::utils;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub enum Network {
    Mainnet,
    Testnet,
}

#[derive(Deserialize)]
pub enum CredentialKind {
    Key,
    Script,
}

#[derive(Deserialize)]
pub struct PaymentCredential {
    pub bech32: String,
    pub hex: String,
    pub kind: CredentialKind,
}

#[derive(Deserialize)]
pub struct StakingCredential {
    pub bech32: String,
    pub hex: String,
    pub kind: CredentialKind,
    pub pointer: Option<HashMap<String, serde_json::Value>>,
    pub reward_address: String,
}

#[derive(Deserialize)]
pub struct DecodedAddress {
    pub bech32: String,
    pub hex: String,
    pub network: Network,
    pub payment_cred: PaymentCredential,
    pub staking_cred: StakingCredential,
}

#[derive(Deserialize)]
pub struct AddressTransactionCount {
    pub data: i32,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize)]
pub struct Transaction {
    pub input: bool,
    pub output: bool,
    pub slot: i64,
    pub tx_hash: String,
}

#[derive(Deserialize)]
pub struct AddressTransactions {
    pub data: Vec<Transaction>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct UtxoReference {
    pub index: i64,
    pub tx_hash: String,
}

#[derive(Deserialize)]
pub struct UtxoReferencesAtAddress {
    pub data: Vec<UtxoReference>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct Asset {
    pub amount: i64,
    pub unit: String,
}

#[derive(Deserialize)]
pub struct Utxo {
    pub address: String,
    pub assets: Vec<Asset>,
    pub datum: Option<HashMap<String, serde_json::Value>>,
    pub index: i64,
    pub reference_script: ReferenceScript,
    pub tx_hash: String,
    pub txout_cbor: String,
}

#[derive(Deserialize)]
pub struct ReferenceScript {
    pub bytes: String,
    pub hash: String,
    pub json: Option<HashMap<String, serde_json::Value>>,
    pub r#type: String,
}

#[derive(Deserialize)]
pub struct UtxosAtAddress {
    pub data: Vec<Utxo>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}
