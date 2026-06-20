use crate::cex::{
    price_basis::PriceBasis, trigger_direction::TriggerDirection, trigger_mode::TriggerMode,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Hash, Serialize, Deserialize)]
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
        callback_rate_bps: N,
        direction: TriggerDirection,
    },
}
