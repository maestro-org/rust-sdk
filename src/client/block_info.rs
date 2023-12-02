use super::maestro::Maestro;
use crate::utils;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct OperationalCertificate {
    pub hot_vkey: String,
    pub kes_period: i64,
    pub kes_signature: String,
    pub sequence_number: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TotalExUnits {
    pub mem: i64,
    pub steps: i64,
}

#[derive(Serialize, Deserialize)]
pub struct BlockInfoData {
    pub absolute_slot: i64,
    pub block_producer: String,
    pub confirmations: i64,
    pub epoch: i64,
    pub epoch_slot: i64,
    pub era: String,
    pub hash: String,
    pub height: i64,
    pub operational_certificate: OperationalCertificate,
    pub previous_block: String,
    pub protocol_version: Vec<i32>,
    pub script_invocations: i32,
    pub size: i32,
    pub timestamp: String,
    pub total_ex_units: TotalExUnits,
    pub total_fees: i64,
    pub total_output_lovelace: String,
    pub tx_hashes: Vec<String>,
    pub vrf_key: String,
}

#[derive(Deserialize)]
pub struct BlockInfo {
    pub data: BlockInfoData,
    pub last_updated: utils::last_updated::LastUpdated,
}

impl Maestro {
    pub async fn block_info(&self, block_height: i64) -> Result<BlockInfo, Box<dyn Error>> {
        let url = format!("/blocks/{}", block_height);

        let response = self.get(&url).await?;

        let block_info =
            serde_json::from_str(&response).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(block_info)
    }
}
