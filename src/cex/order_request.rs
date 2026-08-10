use crate::cex::orders::single_order::SingleOrder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use strum::Display;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Hash)]
pub enum OrderRequest<Asset, Number> {
    Single(SingleOrder<Asset, Number>),
}
