#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

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
    type Result = (TokenPair, Amm);
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        let TokenPairPool {
            pair: (token0, token1),
            amm,
        } = &self.0;
        let pair = TokenPair::new(*token0, *token1);

        // check supported token
        if !with_state(|s| s.business_tokens_query().contains_key(&pair.token0)) {
            return Err(BusinessError::NotSupportedToken(pair.token0));
        }
        if !with_state(|s| s.business_tokens_query().contains_key(&pair.token1)) {
            return Err(BusinessError::NotSupportedToken(pair.token1));
        }

        // parse amm
        let amm: Amm = amm.try_into()?;

        // check exist
        if with_state(|s| s.business_token_pair_pool_exist(&pair, &amm)) {
            return Err(BusinessError::NotSupportedToken(pair.token1));
        }

        Ok((pair, amm))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_create")]
async fn pair_create(args: TokenPairPool) -> BusinessResult {
    inner_pair_create(args.into()).await.into()
}
async fn inner_pair_create(args: TokenPairCreateArgs) -> Result<(), BusinessError> {
    // 1. check args
    let (pair, amm) = args.check_args()?;

    // 2. some value

    with_mut_state_without_record(|s| s.business_token_pair_pool_create(pair, amm))
}
