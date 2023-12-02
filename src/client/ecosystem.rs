use super::maestro::Maestro;
use crate::models::common::BasicResponse;
use std::error::Error;

impl Maestro {
    pub async fn resolve_ada_handle(&self, handle: &str) -> Result<BasicResponse, Box<dyn Error>> {
        let url = format!("/ecosystem/adahandle/{}", handle);
        let resp = self.get(&url).await?;

        let ada_handle = serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(ada_handle)
    }
}
