use crate::cex::capability::CexCapability;

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    strum::Display,
    strum_macros::EnumString,
};

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Display, EnumString, Serialize, Deserialize))]
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
