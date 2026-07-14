use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Quantity<N> {
    OfBase(N),
    OfQuote(N),
}
