#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== create ==========================

// create
impl CheckArgs for TokenPairCreateArgs {
    type Result = (TimestampNanos, Caller, TokenPairAmm);
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        let TokenPairPool {
            token0: token_a,
            token1: token_b,
            amm,
        } = &self.pool;
        let pair = TokenPair::new(*token_a, *token_b);
        pair.check_args()?; // check supported token
        let amm: Amm = amm.as_ref().try_into()?; // parse amm

        let pa = TokenPairAmm { pair, amm };

        // check exist
        if with_state(|s| s.business_token_pair_pool_get(&pa).is_some()) {
            return Err(BusinessError::TokenPairAmmExist(pa));
        }

        // check meta
        let now = check_meta(&self.memo, &self.created)?;

        Ok((now, Caller::get(), pa))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_create")]
async fn pair_create(args: TokenPairCreateArgs) -> TokenPairCreateResult {
    inner_pair_create(args).await.map(|m| m.into()).into()
}
async fn inner_pair_create(args: TokenPairCreateArgs) -> Result<MarketMaker, BusinessError> {
    // 1. check args
    let (now, caller, pa) = args.check_args()?;

    // 2. some value

    let maker = {
        // 3. lock
        let lock = match super::super::lock_swap_block_chain(0)? {
            LockResult::Locked(lock) => lock,
            LockResult::Retry(_) => return Err(BusinessError::SwapBlockChainLocked),
        };

        // * 4. do business
        {
            with_mut_state_without_record(|s| {
                s.business_token_pair_pool_create(
                    &lock,
                    ArgWithMeta {
                        now,
                        caller,
                        arg: pa,
                        memo: args.memo,
                        created: args.created,
                    },
                )
            })?
        }
    };

    // TODO 异步触发同步任务

    Ok(maker)
}
