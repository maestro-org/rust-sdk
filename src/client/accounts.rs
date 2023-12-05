use super::maestro::Maestro;
use crate::{
    models::accounts::{
        AccountAddresses, AccountAssets, StakeAccountHistory, StakeAccountInformation,
        StakeAccountRewards, StakeAccountUpdates,
    },
    utils::Parameters,
};
use std::error::Error;

impl Maestro {
    pub async fn account_addresses(
        &self,
        stake_addr: &str,
        params: Option<Parameters>,
    ) -> Result<AccountAddresses, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/accounts/{}/addresses{}", stake_addr, formatted_params);
        let resp = self.get(&url).await?;
        let account_addresses =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(account_addresses)
    }

    pub async fn account_assets(
        &self,
        stake_addr: &str,
        params: Option<Parameters>,
    ) -> Result<AccountAssets, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/accounts/{}/assets{}", stake_addr, formatted_params);
        let resp = self.get(&url).await?;
        let account_assets =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(account_assets)
    }

    pub async fn stake_account_history(
        &self,
        stake_addr: &str,
        params: Option<Parameters>,
    ) -> Result<StakeAccountHistory, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/accounts/{}/history{}", stake_addr, formatted_params);
        let resp = self.get(&url).await?;
        let stake_account_history =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(stake_account_history)
    }

    pub async fn stake_account_information(
        &self,
        stake_addr: &str,
    ) -> Result<StakeAccountInformation, Box<dyn Error>> {
        let url = format!("/accounts/{}", stake_addr);
        let resp = self.get(&url).await?;
        let stake_account_information =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(stake_account_information)
    }

    pub async fn stake_account_rewards(
        &self,
        stake_addr: &str,
        params: Option<Parameters>,
    ) -> Result<StakeAccountRewards, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/accounts/{}/rewards{}", stake_addr, formatted_params);
        let resp = self.get(&url).await?;
        let stake_account_rewards =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(stake_account_rewards)
    }

    pub async fn stake_account_updates(
        &self,
        stake_addr: &str,
        params: Option<Parameters>,
    ) -> Result<StakeAccountUpdates, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/accounts/{}/updates{}", stake_addr, formatted_params);
        let resp = self.get(&url).await?;
        let stake_account_updates =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(stake_account_updates)
    }
}
