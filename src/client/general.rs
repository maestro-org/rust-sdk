use super::maestro::Maestro;
use crate::models::{
    common::BasicResponse,
    general::{ChainTip, EraHistory, ProtocolParameters},
};
use std::error::Error;

impl Maestro {
    pub async fn chain_tip(&self) -> Result<ChainTip, Box<dyn Error>> {
        let url = "/chain-tip";
        let resp = self.get(url).await?;
        let chain_tip = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(chain_tip)
    }

    pub async fn era_history(&self) -> Result<EraHistory, Box<dyn Error>> {
        let url = "/era-history";
        let resp = self.get(url).await?;
        let era_history = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(era_history)
    }

    pub async fn protocol_parameters(&self) -> Result<ProtocolParameters, Box<dyn Error>> {
        let url = "/protocol-params";
        let resp = self.get(url).await?;
        let protocol_params =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(protocol_params)
    }

    pub async fn block_chain_start_time(&self) -> Result<BasicResponse, Box<dyn Error>> {
        let url = "/system-start";
        let resp = self.get(url).await?;
        let blockchain_start_time =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(blockchain_start_time)
    }
}
