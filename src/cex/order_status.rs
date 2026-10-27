#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
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
