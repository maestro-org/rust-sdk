use super::maestro::Maestro;
use crate::models::datum::DatumFromHash;
use std::error::Error;

impl Maestro {
    pub async fn datum_from_hash(&self, hash: &str) -> Result<DatumFromHash, Box<dyn Error>> {
        let url = format!("/data/{}", hash);
        let resp = self.get(&url).await?;
        let datum = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(datum)
    }
}
