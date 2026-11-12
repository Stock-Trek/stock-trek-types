#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use strum::Display;

#[derive(Debug, Display, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Quantity<N> {
    OfBase(N),
    OfQuote(N),
}
