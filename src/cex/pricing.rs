use crate::cex::time_in_force::TimeInForce;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Hash, Serialize, Deserialize)]
pub enum Pricing<N> {
    Market,
    Limit {
        price: N,
        time_in_force: TimeInForce,
    },
}
