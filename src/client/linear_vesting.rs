use super::maestro::Maestro;
use crate::models::linear_vesting::{CollectTransaction, LockTransaction, VestingState};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct LockBody {
    pub sender: String,
    pub beneficiary: String,
    pub asset_policy_id: String,
    pub asset_token_name: String,
    pub total_vesting_quantity: i64,
    pub vesting_period_start: i64,
    pub vesting_period_end: i64,
    pub first_unlock_possible_after: i64,
    pub total_installments: i64,
}

impl Maestro {
    pub async fn lock_assets(
        &self,
        lock_body: LockBody,
    ) -> Result<LockTransaction, Box<dyn Error>> {
        let url = "/contracts/vesting/lock";
        let resp = self.post(url, lock_body).await?;
        let lock_transaction =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(lock_transaction)
    }

    pub async fn state_of_vesting_assets(
        &self,
        beneficiary: &str,
    ) -> Result<Vec<VestingState>, Box<dyn Error>> {
        let url = format!("/contracts/vesting/state/{}", beneficiary);
        let resp = self.get(&url).await?;
        let vesting_states =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(vesting_states)
    }

    pub async fn collect_assets(
        &self,
        beneficiary: &str,
    ) -> Result<CollectTransaction, Box<dyn Error>> {
        let url = format!("/contracts/vesting/collect/{}", beneficiary);
        let resp = self.post(&url, beneficiary).await?;
        let collect_transaction =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(collect_transaction)
    }
}
