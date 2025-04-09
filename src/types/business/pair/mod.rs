use super::*;

mod create;
pub use create::*;

mod liquidity;
pub use liquidity::*;

mod swap;
pub use swap::*;

mod loan;
pub use loan::*;

/// (token0, token1, amm)
#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairPool {
    pub pair: (CanisterId, CanisterId),
    pub amm: AmmText,
}

impl TokenPair {
    pub fn to_pool(self, amm: &Amm) -> TokenPairPool {
        TokenPairPool {
            pair: (self.token0, self.token1),
            amm: amm.into(),
        }
    }
}
