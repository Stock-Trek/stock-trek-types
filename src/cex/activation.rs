use crate::cex::{
    price_basis::PriceBasis, trigger_direction::TriggerDirection, trigger_mode::TriggerMode,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use strum::Display;

#[derive(Debug, Display, Clone, Copy, Hash)]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Activation<N> {
    Immediate,
    PriceTriggered {
        activation_price: N,
        basis: PriceBasis,
        direction: TriggerDirection,
        mode: TriggerMode,
    },
    Trailing {
        activation_price: N,
        basis: PriceBasis,
        callback_rate_bps: u32,
        direction: TriggerDirection,
    },
}
