use super::maestro::Maestro;
use crate::{
    models::asset::{
        AccountsHoldingAsset, AddressesHoldingAsset, AssetInformations, AssetTransactions,
        AssetUpdates, AssetUtxos,
    },
    utils::Parameters,
};
use std::error::Error;

impl Maestro {
    pub async fn accounts_holding_asset(
        &self,
        asset_id: &str,
        params: Option<&Parameters>,
    ) -> Result<AccountsHoldingAsset, Box<dyn Error>> {
        let formatted_params = params.map_or_else(|| "".to_string(), |p| p.format());
        let url = format!("/assets/{}/accounts{}", asset_id, formatted_params);
        let resp = self.get(&url).await?;
        let accounts_holding_asset =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(accounts_holding_asset)
    }

    pub async fn address_holding_asset(
        &self,
        asset_id: &str,
        params: Option<&Parameters>,
    ) -> Result<AddressesHoldingAsset, Box<dyn Error>> {
        let formatted_params = params.map_or_else(|| "".to_string(), |p| p.format());
        let url = format!("/assets/{}/addresses{}", asset_id, formatted_params);
        let resp = self.get(&url).await?;
        let addresses_holding_asset =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(addresses_holding_asset)
    }

    pub async fn asset(&self, asset_id: &str) -> Result<AssetInformations, Box<dyn Error>> {
        let url = format!("/assets/{}", asset_id);
        let resp = self.get(&url).await?;
        let asset_information =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(asset_information)
    }

    pub async fn asset_transactions(
        &self,
        asset_id: &str,
        params: Option<&Parameters>,
    ) -> Result<AssetTransactions, Box<dyn Error>> {
        let formatted_params = params.map_or_else(|| "".to_string(), |p| p.format());
        let url = format!("/assets/{}/txs{}", asset_id, formatted_params);
        let resp = self.get(&url).await?;
        let asset_transactions =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(asset_transactions)
    }

    pub async fn asset_updates(
        &self,
        asset_id: &str,
        params: Option<&Parameters>,
    ) -> Result<AssetUpdates, Box<dyn Error>> {
        let formatted_params = params.map_or_else(|| "".to_string(), |p| p.format());
        let url = format!("/assets/{}/updates{}", asset_id, formatted_params);
        let resp = self.get(&url).await?;
        let asset_updates =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(asset_updates)
    }

    pub async fn asset_utxos(
        &self,
        asset_id: &str,
        params: Option<&Parameters>,
    ) -> Result<AssetUtxos, Box<dyn Error>> {
        let formatted_params = params.map_or_else(|| "".to_string(), |p| p.format());
        let url = format!("/assets/{}/utxos{}", asset_id, formatted_params);
        let resp = self.get(&url).await?;
        let asset_utxos = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(asset_utxos)
    }
}
