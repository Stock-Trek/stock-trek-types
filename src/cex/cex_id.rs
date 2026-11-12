use crate::cex::capability::CexCapability;
use strum::{Display, EnumCount, EnumIter, EnumString};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Display,
    EnumCount,
    EnumIter,
    EnumString,
    Clone,
    Copy,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Hash,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum CexId {
    Binance,
}

impl CexId {
    pub fn has_capability(&self, _capability: CexCapability) -> bool {
        false
        // matches!((self, capability), (Self::Binance, CexCapability::...))
    }
}
