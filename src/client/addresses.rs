use super::maestro::Maestro;
use crate::{
    models::addresses::{
        AddressTransactionCount, AddressTransactions, DecodedAddress, UtxoReferencesAtAddress,
        UtxosAtAddress,
    },
    utils,
};
use std::error::Error;

impl Maestro {
    pub async fn decode_address(&self, address: &str) -> Result<DecodedAddress, Box<dyn Error>> {
        let url = format!("/addresses/{}/decode/", address);
        let resp = self.get(&url).await?;
        let decoded_address =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(decoded_address)
    }

    pub async fn address_transaction_count(
        &self,
        address: &str,
    ) -> Result<AddressTransactionCount, Box<dyn Error>> {
        let url = format!("/addresses/{}/transactions/count", address);
        let resp = self.get(&url).await?;
        let transaction_count =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(transaction_count)
    }

    pub async fn address_transactions(
        &self,
        address: &str,
        params: Option<&utils::Parameters>,
    ) -> Result<AddressTransactions, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/addresses/{}/transactions{}", address, formatted_params);
        let resp = self.get(&url).await?;
        let address_transactions =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(address_transactions)
    }

    pub async fn payment_credential_transactions(
        &self,
        payment_credential: &str,
        params: Option<utils::Parameters>,
    ) -> Result<AddressTransactions, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!(
            "/addresses/cred/{}/transactions{}",
            payment_credential, formatted_params
        );
        let resp = self.get(&url).await?;
        let address_transactions =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(address_transactions)
    }

    pub async fn utxo_references_at_address(
        &self,
        address: &str,
        params: Option<utils::Parameters>,
    ) -> Result<UtxoReferencesAtAddress, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/addresses/{}/utxo_refs{}", address, formatted_params);
        let resp = self.get(&url).await?;
        let utxo_references =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(utxo_references)
    }

    pub async fn utxos_at_address(
        &self,
        address: &str,
        params: Option<utils::Parameters>,
    ) -> Result<UtxosAtAddress, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/addresses/{}/utxos{}", address, formatted_params);
        let resp = self.get(&url).await?;
        let utxos = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(utxos)
    }

    pub async fn utxos_at_addresses(
        &self,
        addresses: Vec<String>,
        params: Option<utils::Parameters>,
    ) -> Result<UtxosAtAddress, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!("/addresses/utxos{}", formatted_params);
        let resp = self.post(&url, addresses).await?;
        let utxos = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(utxos)
    }

    pub async fn utxos_by_payment_credential(
        &self,
        payment_credential: &str,
        params: Option<utils::Parameters>,
    ) -> Result<UtxosAtAddress, Box<dyn Error>> {
        let formatted_params = params.map(|p| p.format()).unwrap_or_default();
        let url = format!(
            "/addresses/cred/{}/utxos{}",
            payment_credential, formatted_params
        );
        let resp = self.get(&url).await?;
        let utxos = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(utxos)
    }
}
