use crate::cex::capability::CexCapability;
use strum::{Display, EnumCount, EnumIter, EnumString};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, EnumCount, EnumIter, EnumString, Clone, Copy, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
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
