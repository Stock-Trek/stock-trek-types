use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PriceBasis {
    Last,
    Mark,
    Index,
    BestBid,
    BestAsk,
}
