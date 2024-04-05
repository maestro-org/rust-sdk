use super::maestro::Maestro;
use crate::models::{
    common::BasicResponse,
    transactions::{
        EvaluateTx, RedeemerEvaluation, TransactionDetails, TransactionOutputFromReference,
        TransactionOutputsFromReferences,
    },
};
use std::{collections::HashMap, error::Error};

impl Maestro {
    pub async fn address_by_output_reference(
        &self,
        tx_hash: &str,
        index: i32,
    ) -> Result<BasicResponse, Box<dyn Error>> {
        let url = format!("/transactions/{}/outputs/{}/address", tx_hash, index);
        let resp = self.get(&url).await?;
        let address_by_output_reference =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(address_by_output_reference)
    }

    pub async fn submit_tx(&self, cbor: &str) -> Result<BasicResponse, Box<dyn Error>> {
        let url = "/submit/tx";
        let resp = self.post(url, cbor.to_string()).await?;
        let submit_tx = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(submit_tx)
    }

    pub async fn transaction_cbor(&self, tx_hash: &str) -> Result<BasicResponse, Box<dyn Error>> {
        let url = format!("/transactions/{}/cbor", tx_hash);
        let resp = self.get(&url).await?;
        let transaction_cbor =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(transaction_cbor)
    }

    pub async fn transaction_details(
        &self,
        tx_hash: &str,
    ) -> Result<TransactionDetails, Box<dyn Error>> {
        let url = format!("/transactions/{}", tx_hash);
        let resp = self.get(&url).await?;
        let transaction_details =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(transaction_details)
    }

    pub async fn transaction_output_from_reference(
        &self,
        tx_hash: &str,
        index: i32,
        params: Option<HashMap<String, String>>,
    ) -> Result<TransactionOutputFromReference, Box<dyn Error>> {
        let formatted_params = params.map_or("".to_string(), |p| {
            "?".to_string() + p.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("&")
                .as_str()
        });
        let url = format!(
            "/transactions/{}/outputs/{}/txo{}",
            tx_hash, index, formatted_params
        );
        let resp = self.get(&url).await?;
        let transaction_output_from_reference =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(transaction_output_from_reference)
    }

    pub async fn transaction_outputs_from_references(
        &self,
        references: Vec<String>,
        params: Option<HashMap<String, String>>,
    ) -> Result<TransactionOutputsFromReferences, Box<dyn Error>> {
        let formatted_params = params.map_or("".to_string(), |p| {
            p.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("&")
                .to_string()
        });
        let url = format!("/transactions/outputs{}", formatted_params);
        let resp = self.post(&url, references).await?;
        let transaction_outputs_from_references =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(transaction_outputs_from_references)
    }

    pub async fn evaluate_tx(
        &self,
        tx_cbor: &str,
        additional_utxos: Vec<String>,
    ) -> Result<Vec<RedeemerEvaluation>, Box<dyn Error>> {
        let url = "/transactions/evaluate";
        let body = EvaluateTx {
            cbor: tx_cbor.to_string(),
            additional_utxos,
        };
        let resp = self.post(url, &body).await?;
        let redeemer_evaluations =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(redeemer_evaluations)
    }
}
