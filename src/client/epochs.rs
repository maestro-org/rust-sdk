use super::maestro::Maestro;
use crate::models::epochs::EpochResp;
use std::error::Error;

impl Maestro {
    pub async fn current_epoch(&self) -> Result<EpochResp, Box<dyn Error>> {
        let url = "/epochs/current";
        let resp = self.get(url).await?;
        let current_epoch =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(current_epoch)
    }

    pub async fn specific_epoch(&self, epoch_no: i32) -> Result<EpochResp, Box<dyn Error>> {
        let url = format!("/epochs/{}", epoch_no);
        let resp = self.get(&url).await?;
        let specific_epoch =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(specific_epoch)
    }
}
