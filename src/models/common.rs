use crate::utils::LastUpdated;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BasicResponse {
    pub data: String,
    pub last_updated: LastUpdated,
}
