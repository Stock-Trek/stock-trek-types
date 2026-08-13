#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use strum::Display;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", derive(Display))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeInForce {
    GoodTillCancelled,
    // TODO
    // GoodTillTime(TimestampMillis),
    FillOrKill,
    ImmediateOrCancel,
}
