use crate::cex::capability::CexCapability;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::Display;
#[cfg(feature = "serde")]
use strum_macros::EnumString;

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(EnumString, Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
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
