use super::maestro::Maestro;
use crate::{
    models::pools::{
        PoolMintedBlocks, RegisteredPools, StakePoolDelegators, StakePoolHistory,
        StakePoolInformation, StakePoolMetadata, StakePoolRelays, StakePoolUpdates,
    },
    utils::Parameters,
};
use std::error::Error;

impl Maestro {
    pub async fn list_of_registered_pools(
        &self,
        params: Option<Parameters>,
    ) -> Result<RegisteredPools, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/pools{}", formatted_params);
        let resp = self.get(&url).await?;
        let registered_pools =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(registered_pools)
    }

    pub async fn stake_pool_minted_blocks(
        &self,
        pool_id: &str,
        params: Option<Parameters>,
    ) -> Result<PoolMintedBlocks, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/pools/{}/blocks{}", pool_id, formatted_params);
        let resp = self.get(&url).await?;
        let pool_minted_blocks =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(pool_minted_blocks)
    }

    pub async fn stake_pool_delegators(
        &self,
        pool_id: &str,
        params: Option<Parameters>,
    ) -> Result<StakePoolDelegators, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/pools/{}/delegators{}", pool_id, formatted_params);
        let resp = self.get(&url).await?;
        let stake_pool_delegators =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(stake_pool_delegators)
    }

    pub async fn stake_pool_history(
        &self,
        pool_id: &str,
        params: Option<Parameters>,
    ) -> Result<StakePoolHistory, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/pools/{}/history{}", pool_id, formatted_params);
        let resp = self.get(&url).await?;
        let stake_pool_history =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(stake_pool_history)
    }

    pub async fn stake_pool_information(
        &self,
        pool_id: &str,
    ) -> Result<StakePoolInformation, Box<dyn Error>> {
        let url = format!("/pools/{}/info", pool_id);
        let resp = self.get(&url).await?;
        let stake_pool_information =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(stake_pool_information)
    }

    pub async fn stake_pool_metadata(
        &self,
        pool_id: &str,
    ) -> Result<StakePoolMetadata, Box<dyn Error>> {
        let url = format!("/pools/{}/metadata", pool_id);
        let resp = self.get(&url).await?;
        let stake_pool_metadata =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(stake_pool_metadata)
    }

    pub async fn stake_pool_relays(
        &self,
        pool_id: &str,
    ) -> Result<StakePoolRelays, Box<dyn Error>> {
        let url = format!("/pools/{}/relays", pool_id);
        let resp = self.get(&url).await?;
        let stake_pool_relays =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(stake_pool_relays)
    }

    pub async fn stake_pool_updates(
        &self,
        pool_id: &str,
    ) -> Result<StakePoolUpdates, Box<dyn Error>> {
        let url = format!("/pools/{}/updates", pool_id);
        let resp = self.get(&url).await?;
        let stake_pool_updates =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(stake_pool_updates)
    }

    pub async fn stake_pool_delegator_history(
        &self,
        pool_id: &str,
        epoch_no: i64,
        params: Option<Parameters>,
    ) -> Result<StakePoolUpdates, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/pools/{}/delegators/{}{}", pool_id, epoch_no, formatted_params);
        let resp = self.get(&url).await?;
        let stake_pool_delegator_history =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(stake_pool_delegator_history)
    }
}
