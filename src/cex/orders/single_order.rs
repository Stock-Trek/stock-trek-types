use crate::cex::{
    activation::Activation, pricing::Pricing, quantity::Quantity, side::Side, tag::Tag,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash)]
pub struct SingleOrder<Asset, Number> {
    pub base: Asset,
    pub quote: Asset,
    pub activation: Activation<Number>,
    pub pricing: Pricing<Number>,
    pub side: Side,
    pub quantity: Quantity<Number>,
    pub tag: Tag,
}
