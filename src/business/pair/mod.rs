#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

mod liquidity;

impl CheckArgs for TokenPair {
    type Result = ();

    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check supported token
        with_state(|s| {
            let tokens = s.business_tokens_query();
            if !tokens.contains_key(&self.token0) {
                return Err(BusinessError::NotSupportedToken(self.token0));
            }
            if !tokens.contains_key(&self.token1) {
                return Err(BusinessError::NotSupportedToken(self.token1));
            }
            Ok(())
        })
    }
}

// ========================== query pairs ==========================

// anyone can query
#[ic_cdk::query]
fn pairs_query() -> Vec<(TokenPairPool, MarketMakerView)> {
    with_state(|s| {
        s.business_token_pair_pools_query()
            .into_iter()
            .map(|(pair, amm, maker)| (pair.to_pool(amm), maker.clone().into()))
            .collect()
    })
}

// ========================== create ==========================

// create
impl CheckArgs for TokenPairCreateArgs {
    type Result = PairAmm;
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        let TokenPairPool {
            pair: (token_a, token_b),
            amm,
        } = &self.0;
        let pair = TokenPair::new(*token_a, *token_b);
        pair.check_args()?; // check supported token
        let amm: Amm = amm.try_into()?; // parse amm

        let pa = PairAmm { pair, amm };

        // check exist
        if with_state(|s| s.business_token_pair_pool_maker_get(&pa).is_some()) {
            return Err(BusinessError::TokenPairAmmExist((pair, (&amm).into())));
        }

        Ok(pa)
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_create")]
async fn pair_create(args: TokenPairPool) -> BusinessResult {
    inner_pair_create(args.into()).await.into()
}
async fn inner_pair_create(args: TokenPairCreateArgs) -> Result<(), BusinessError> {
    // 1. check args
    let pa = args.check_args()?;

    // 2. some value

    with_mut_state_without_record(|s| s.business_token_pair_pool_create(pa))
}
