use common::types::{SelfCanister, SwapTokenPair, TokenPairAmm};

mod create_or_remove;
pub use create_or_remove::*;

mod liquidity;
pub use liquidity::*;

mod swap;
pub use swap::*;

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
