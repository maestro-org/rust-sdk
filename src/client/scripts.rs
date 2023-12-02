use super::maestro::Maestro;
use crate::models::scripts::ScriptByHash;
use std::error::Error;

impl Maestro {
    pub async fn script_by_hash(&self, hash: &str) -> Result<ScriptByHash, Box<dyn Error>> {
        let url = format!("/scripts/{}", hash);
        let resp = self.get(&url).await?;

        let script_by_hash =
            serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(script_by_hash)
    }
}
