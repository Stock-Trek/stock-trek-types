#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Clone, Copy, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum AssetId {
    Aave,
    Arbitrum,
    Atom,
    Avalanche,
    Bitcoin,
    BitcoinCash,
    BNB,
    Celo,
    Chainlink,
    Cronos,
    Dai,
    Dogecoin,
    Ethereum,
    Fantom,
    Gnosis,
    Litecoin,
    Moonbeam,
    NEAR,
    Optimism,
    Osmosis,
    Polygon,
    Solana,
    TetherUSD,
    TRON,
    Uniswap,
    USDCoin,
    WrappedBitcoin,
    WrappedEthereum,
}
