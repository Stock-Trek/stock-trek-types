use crate::cex::time_in_force::TimeInForce;
use rust_decimal::Decimal;
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

impl Eq for Pricing<Decimal> {}
impl PartialEq for Pricing<Decimal> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Pricing::Market => match other {
                Pricing::Market => true,
                Pricing::Limit { .. } => false,
            },
            Pricing::Limit {
                price,
                time_in_force,
            } => match other {
                Pricing::Market => false,
                Pricing::Limit {
                    price: o_price,
                    time_in_force: o_time_in_force,
                } => price == o_price && time_in_force == o_time_in_force,
            },
        }
    }
}
