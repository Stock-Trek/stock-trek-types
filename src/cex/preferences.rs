use crate::util::serde_rounding_strategy;
use rust_decimal::RoundingStrategy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CexPreferences {
    pub max_network_delay_millis: u32,
    pub rounding: CexRoundingPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CexRoundingPreferences {
    #[serde(with = "serde_rounding_strategy")]
    pub activation_price_triggered_above: RoundingStrategy,
    #[serde(with = "serde_rounding_strategy")]
    pub activation_price_triggered_below: RoundingStrategy,
    #[serde(with = "serde_rounding_strategy")]
    pub price: RoundingStrategy,
    #[serde(with = "serde_rounding_strategy")]
    pub quantity: RoundingStrategy,
    #[serde(with = "serde_rounding_strategy")]
    pub callback_rate_bps: RoundingStrategy,
}
