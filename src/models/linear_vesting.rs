use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LockTransaction {
    pub cbor_hex: String,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct VestingState {
    pub asset_name: String,
    pub asset_symbol: String,
    pub remaining_vesting_quantity: i64,
    pub total_installments: i64,
    pub total_vesting_quantity: i64,
    pub vesting_period_start: i64,
    pub vesting_period_end: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CollectTransaction {
    pub cbor_hex: String,
    pub tx_hash: String,
}
