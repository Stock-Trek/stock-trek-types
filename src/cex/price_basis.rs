#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::Display;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PriceBasis {
    Last,
    Mark,
    Index,
    BestBid,
    BestAsk,
}
