#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use strum::Display;

#[derive(Debug, Display, Clone, Copy, Hash)]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Quantity<N> {
    OfBase(N),
    OfQuote(N),
}
