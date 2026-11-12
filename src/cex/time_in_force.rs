#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum TimeInForce {
    GoodTillCancelled,
    // TODO
    // GoodTillTime(TimestampMillis),
    FillOrKill,
    ImmediateOrCancel,
}
