use crate::cex::{
    activation::Activation, pricing::Pricing, quantity::Quantity, side::Side, tag::Tag,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SingleOrder<Asset, Number> {
    pub base: Asset,
    pub quote: Asset,
    pub activation: Activation<Number>,
    pub pricing: Pricing<Number>,
    pub side: Side,
    pub quantity: Quantity<Number>,
    pub tag: Tag,
}
