#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use strum_macros::{Display, EnumString};

#[cfg_attr(feature = "serde", derive(Display, EnumString, Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderStatus {
    Canceled,
    Expired,
    Filled,
    New,
    PartiallyFilled,
    PendingCancel,
    Rejected,
    Unknown,
}
