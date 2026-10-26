#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::Display;
#[cfg(feature = "serde")]
use strum_macros::EnumString;

#[cfg_attr(feature = "serde", derive(EnumString, Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerMode {
    Touch,
    Cross,
}
