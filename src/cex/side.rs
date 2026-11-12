#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Side {
    Buy,
    Sell,
}
