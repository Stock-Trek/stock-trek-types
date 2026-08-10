#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::Display;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
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
