use crate::utils;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Datum {
    pub bytes: String,
    pub json: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatumFromHash {
    pub data: Datum,
    pub last_updated: utils::LastUpdated,
}
