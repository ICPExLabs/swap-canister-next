#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== remove ==========================

// remove
fn check_pair_remove_args(
    _self: &TokenPairCreateOrRemoveArgs,
) -> Result<(TimestampNanos, Caller, TokenPairAmm), BusinessError> {
    // ! refuse all action about frozen token
    with_state(|s| s.business_token_alive(&_self.pool.token0))?;
    with_state(|s| s.business_token_alive(&_self.pool.token1))?;

    let TokenPairPool {
        token0: token_a,
        token1: token_b,
        amm,
    } = &_self.pool;
    let pair = TokenPair::new(*token_a, *token_b);
    check_token_pair_args(&pair)?; // check supported token
    let amm: Amm = amm.as_ref().try_into()?; // parse amm

    let pa = TokenPairAmm { pair, amm };

    // check exist and removable
    match with_state(|s| s.business_token_pair_pool_get(&pa)) {
        Some(maker) if !maker.removable() => return Err(BusinessError::TokenPairAmmStillAlive(pa)),
        None => return Err(BusinessError::TokenPairAmmNotExist(pa)),
        _ => {}
    }

    // check meta
    let now = check_meta(&_self.memo, &_self.created)?;

    Ok((now, Caller::get(), pa))
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_create_or_remove")]
async fn pair_remove(args: TokenPairCreateOrRemoveArgs) -> TokenPairCreateOrRemoveResult {
    inner_pair_remove(args).await.map(|m| m.into()).into()
}
async fn inner_pair_remove(args: TokenPairCreateOrRemoveArgs) -> Result<MarketMaker, BusinessError> {
    // 1. check args
    let (now, caller, pa) = check_pair_remove_args(&args)?;

    // 2. some value

    let maker = {
        // 3. lock
        let lock = match super::super::lock_swap_block_chain(0)? {
            LockResult::Locked(lock) => lock,
            LockResult::Retry(_) => return Err(BusinessError::SwapBlockChainLocked),
        };

        // * 4. do business
        {
            with_mut_state(|s| {
                s.business_token_pair_pool_remove(
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

    // Asynchronously triggers synchronization tasks
    crate::business::config::push::inner_push_blocks(false, true);

    Ok(maker)
}
