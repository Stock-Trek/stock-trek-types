use crate::cex::{
    price_basis::PriceBasis, trigger_direction::TriggerDirection, trigger_mode::TriggerMode,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use strum::Display;

#[cfg_attr(feature = "serde", derive(Display, Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash)]
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
