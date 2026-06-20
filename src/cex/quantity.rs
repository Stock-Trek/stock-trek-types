use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Hash, Serialize, Deserialize)]
pub enum Quantity<N> {
    OfBase(N),
    OfQuote(N),
}
