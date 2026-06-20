use crate::cex::orders::single_order::SingleOrder;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use strum::Display;

#[derive(Debug, Display, Clone, Hash, Serialize, Deserialize)]
pub enum OrderRequest<A, N> {
    Single(SingleOrder<A, N>),
}
