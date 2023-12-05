use super::maestro::Maestro;
use crate::{
    models::asset_policy::{
        AccountsHoldingPolicy, AddressesHoldingPolicy, PolicyInformation, PolicyTransactions,
        UtxosContainingPolicy,
    },
    utils,
};
use std::error::Error;

impl Maestro {
    pub async fn accounts_holding_policy(
        &self,
        policy: &str,
        params: Option<utils::Parameters>,
    ) -> Result<AccountsHoldingPolicy, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/assets/policy/{}/accounts?{}", policy, formatted_params);
        let resp = self.get(&url).await?;
        let response = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(response)
    }

    pub async fn addresses_holding_policy(
        &self,
        policy: &str,
        params: Option<utils::Parameters>,
    ) -> Result<AddressesHoldingPolicy, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/assets/policy/{}/addresses?{}", policy, formatted_params);
        let resp = self.get(&url).await?;
        let response = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(response)
    }

    pub async fn specific_policy_informations(
        &self,
        policy: &str,
        params: Option<utils::Parameters>,
    ) -> Result<PolicyInformation, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/assets/policy/{}/?{}", policy, formatted_params);
        let resp = self.get(&url).await?;
        let response = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(response)
    }

    pub async fn transactions_moving_policy(
        &self,
        policy: &str,
        params: Option<utils::Parameters>,
    ) -> Result<PolicyTransactions, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/assets/policy/{}/txs?{}", policy, formatted_params);
        let resp = self.get(&url).await?;
        let response = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(response)
    }

    pub async fn utxos_containing_policy(
        &self,
        policy: &str,
        params: Option<utils::Parameters>,
    ) -> Result<UtxosContainingPolicy, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/assets/policy/{}/utxos?{}", policy, formatted_params);
        let resp = self.get(&url).await?;
        let response = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(response)
    }
}
