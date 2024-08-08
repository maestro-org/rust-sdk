use crate::models::markets::{DexPairOHLC, DexPairOHLCParameters};

use super::maestro::Maestro;
use std::{error::Error, fmt::Display};

impl Maestro {
    /// Returns market activity in candlestick OHLC format for a specific DEX and token pair
    pub async fn markets_dex_pair_ohlc(
        &self,
        dex: impl Display,
        pair: impl Display,
        parameters: Option<DexPairOHLCParameters>,
    ) -> Result<Vec<DexPairOHLC>, Box<dyn Error>> {
        let ps = parameters
            .and_then(|p| serde_qs::to_string(&p).ok())
            .map(|x| format!("?{x}"))
            .unwrap_or_default();
        let url = format!("/markets/dexs/ohlc/{dex}/{pair}{ps}");
        let resp = self.get(&url).await?;

        serde_json::from_str(&resp).map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
