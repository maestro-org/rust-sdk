use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TxManagerState {
    pub block: String,
    pub state: String,
    pub timestamp: String,
    pub transaction_hash: String,
}
