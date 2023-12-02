use super::maestro::Maestro;
use crate::models::{common::BasicResponse, txmanager::TxManagerState};
use std::error::Error;

impl Maestro {
    pub async fn tx_manager_history(&self) -> Result<Vec<TxManagerState>, Box<dyn Error>> {
        let url = "/txmanager/history";
        let resp = self.get(url).await?;
        let tx_manager_states =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(tx_manager_states)
    }

    pub async fn tx_manager_submit(&self, cbor: &str) -> Result<BasicResponse, Box<dyn Error>> {
        let url = "/txmanager";
        let resp = self.post(url, cbor.to_string()).await?;
        let submit_tx = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(submit_tx)
    }

    pub async fn tx_manager_submit_turbo(
        &self,
        cbor: &str,
    ) -> Result<BasicResponse, Box<dyn Error>> {
        let url = "/txmanager/turbosubmit";
        let resp = self.post(url, cbor.to_string()).await?;
        let submit_tx = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(submit_tx)
    }

    pub async fn tx_manager_state(&self, tx_hash: &str) -> Result<TxManagerState, Box<dyn Error>> {
        let url = format!("/txmanager/{}/state", tx_hash);
        let resp = self.get(&url).await?;
        let tx_manager_state =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(tx_manager_state)
    }
}
