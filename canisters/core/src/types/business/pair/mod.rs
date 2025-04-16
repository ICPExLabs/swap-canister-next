mod create;
use common::types::{SelfCanister, SwapTokenPair, TokenPairAmm};
pub use create::*;

mod liquidity;
pub use liquidity::*;

mod swap;
pub use swap::*;

mod loan;
pub use loan::*;

pub trait SelfCanisterArg {
    fn get_self_canister(&self) -> SelfCanister;
}

pub trait TokenPairArg {
    fn get_pa(&self) -> &TokenPairAmm;
}

pub trait TokenPairSwapArg {
    fn get_pas(&self) -> &[TokenPairAmm];

    fn get_path(&self) -> &[SwapTokenPair];
}
