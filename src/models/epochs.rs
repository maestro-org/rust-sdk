use crate::utils;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Epoch {
    pub blk_count: i32,
    pub epoch_no: i32,
    pub fees: String,
    pub start_time: i64,
    pub tx_count: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EpochResp {
    pub data: Epoch,
    pub last_updated: utils::LastUpdated,
}
