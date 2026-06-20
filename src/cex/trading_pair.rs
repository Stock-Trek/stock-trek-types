use crate::cex::asset_id::AssetName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradingPair {
    base: AssetName,
    quote: AssetName,
}
impl TradingPair {
    pub fn new(base: AssetName, quote: AssetName) -> Self {
        if base == quote {
            panic!("Base {} cannot be the same as Quote {}", base, quote);
        }
        Self { base, quote }
    }
}
