#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use strum_macros::{Display, EnumString};

#[cfg_attr(feature = "serde", derive(Display, EnumString, Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
