use crate::cex::capability::CexCapability;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use strum_macros::EnumString;

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(EnumString, Serialize, Deserialize))]
#[cfg_attr(feature = "serde", strum(serialize_all = "snake_case"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
