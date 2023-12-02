use serde::Deserialize;

#[derive(Deserialize)]
pub struct LastUpdated {
    pub block_hash: String,
    pub block_slot: i64,
    pub timestamp: String,
}
