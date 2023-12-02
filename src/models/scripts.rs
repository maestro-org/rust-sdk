use crate::utils;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub enum ScriptVersion {
    PlutusV1,
    PlutusV2,
}

#[derive(Serialize, Deserialize)]
pub struct Script {
    pub bytes: String,
    pub hash: String,
    pub json: serde_json::Value,
    pub r#type: ScriptVersion,
}

#[derive(Deserialize)]
pub struct ScriptByHash {
    pub data: Script,
    pub last_updated: utils::LastUpdated,
}
