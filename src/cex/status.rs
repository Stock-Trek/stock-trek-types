#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    strum::Display,
    strum_macros::EnumString,
};

#[cfg_attr(feature = "serde", derive(Display, EnumString, Serialize, Deserialize))]
#[cfg_attr(feature = "serde", strum(serialize_all = "snake_case"))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    New,
    Open,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
    #[default]
    Unknown,
}
