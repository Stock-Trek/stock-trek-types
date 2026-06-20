use crate::cex::{
    activation::Activation, pricing::Pricing, quantity::Quantity, side::Side, tag::Tag,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SingleOrder<A, N> {
    pub base: A,
    pub quote: A,
    pub activation: Activation<N>,
    pub pricing: Pricing<N>,
    pub side: Side,
    pub quantity: Quantity<N>,
    pub tag: Tag,
}
