#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use strum::Display;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", derive(Display))]
#[derive(Debug, Clone, Copy, Hash)]
pub enum Quantity<N> {
    OfBase(N),
    OfQuote(N),
}
