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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
