use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct DexPairOHLC {
    pub coin_a_change_pct: Decimal,
    pub coin_a_close: Decimal,
    pub coin_a_high: Decimal,
    pub coin_a_low: Decimal,
    pub coin_a_open: Decimal,
    pub coin_a_volume: Decimal,
    pub coin_b_change_pct: Decimal,
    pub coin_b_close: Decimal,
    pub coin_b_high: Decimal,
    pub coin_b_low: Decimal,
    pub coin_b_open: Decimal,
    pub coin_b_volume: Decimal,
    pub count: i64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum Resolution {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1mo")]
    OneMonth,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum Sort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct DexPairOHLCParameters {
    pub resolution: Option<Resolution>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub limit: Option<i64>,
    pub sort: Option<Sort>,
}
