#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use strum::Display;

#[cfg_attr(feature = "serde", derive(Display, Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CexCapability {
    QuoteQuantityOnLimitOrders,
}
