#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::Display;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    New,
    Open,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
}
