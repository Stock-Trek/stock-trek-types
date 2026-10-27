#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use strum_macros::EnumString;

#[cfg_attr(feature = "serde", derive(EnumString, Serialize, Deserialize))]
#[cfg_attr(feature = "serde", strum(serialize_all = "snake_case"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PriceBasis {
    Last,
    Mark,
    Index,
    BestBid,
    BestAsk,
}
