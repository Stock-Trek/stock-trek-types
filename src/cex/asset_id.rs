use strum::{Display, EnumCount, EnumIter, EnumString};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Display,
    EnumCount,
    EnumIter,
    EnumString,
    Clone,
    Copy,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Hash,
)]
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
