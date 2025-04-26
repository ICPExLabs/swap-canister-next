#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== add ==========================

// liquidity add
impl CheckArgs for TokenPairLiquidityAddArgs {
    type Result = (
        TimestampNanos,
        Vec<CanisterId>,
        Vec<TokenAccount>,
        SelfCanister,
        Caller,
        TokenPairLiquidityAddArg,
    );
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // ! refuse all action about frozen token
        with_state(|s| s.business_token_alive(&self.swap_pair.token.0))?;
        with_state(|s| s.business_token_alive(&self.swap_pair.token.1))?;

        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check pool
        let (pa, fee_tokens, required) = check_pool(&self.swap_pair, &self_canister, Some(&self.to))?;

        let arg = TokenPairLiquidityAddArg {
            self_canister,
            pa,
            from: self.from,
            token_a: self.swap_pair.token.0,
            token_b: self.swap_pair.token.1,
            amount_a_desired: self.amount_desired.0.clone(),
            amount_b_desired: self.amount_desired.1.clone(),
            amount_a_min: self.amount_min.0.clone(),
            amount_b_min: self.amount_min.1.clone(),
            to: self.to,
        };

        // check amount
        arg.check_args()?;

        // check balance
        let balance_a = with_state(|s| s.business_token_balance_of(arg.token_a, arg.from));
        if balance_a < arg.amount_a_desired {
            return Err(BusinessError::insufficient_balance(arg.token_a, balance_a));
        }
        let balance_b = with_state(|s| s.business_token_balance_of(arg.token_b, arg.from));
        if balance_b < arg.amount_b_desired {
            return Err(BusinessError::insufficient_balance(arg.token_b, balance_b));
        }

        // check deadline
        if let Some(deadline) = &self.deadline {
            deadline.check_args()?;
        }

        // check meta
        let now = check_meta(&self.memo, &self.created)?;

        Ok((now, fee_tokens, required, self_canister, caller, arg))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_liquidity_add")]
async fn pair_liquidity_add(args: TokenPairLiquidityAddArgs, retries: Option<u8>) -> TokenPairLiquidityAddResult {
    inner_pair_liquidity_add(args, retries).await.into()
}
#[inline]
async fn inner_pair_liquidity_add(
    args: TokenPairLiquidityAddArgs,
    retries: Option<u8>,
) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
    // 1. check args
    let (now, fee_tokens, mut required, self_canister, caller, arg) = args.check_args()?;

    // 2. some value
    // let fee_tokens = vec![];
    let token_account_a = TokenAccount::new(arg.token_a, arg.from);
    let token_account_b = TokenAccount::new(arg.token_b, arg.from);
    required.push(token_account_a);
    required.push(token_account_b);

    let success = {
        // 3. lock
        let locks = match super::super::super::lock_token_block_chain_and_swap_block_chain_and_token_balances(
            fee_tokens,
            required,
            retries.unwrap_or_default(),
        )? {
            LockResult::Locked(locks) => locks,
            LockResult::Retry(retries) => {
                return retry_pair_liquidity_add(self_canister.id(), args, retries).await;
            }
        };

        // * 4. do business
        {
            with_mut_state(|s| {
                s.business_token_pair_liquidity_add(
                    &locks,
                    ArgWithMeta {
                        now,
                        caller,
                        arg,
                        memo: args.memo,
                        created: args.created,
                    },
                )
            })?
        }
    };

    // Asynchronously triggers synchronization tasks
    crate::business::config::push::inner_push_blocks(true, true);

    Ok(success)
}
// ! This implicitly contains self_canister_id, which can be called again through permission checks and replaces caller.
#[inline]
async fn retry_pair_liquidity_add(
    self_canister_id: CanisterId,
    args: TokenPairLiquidityAddArgs,
    retries: u8,
) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
    ic_cdk::println!("🔄 retry_pair_liquidity_add: {}", retries);
    let service_swap = crate::services::swap::Service(self_canister_id);
    return service_swap.pair_liquidity_add(args, Some(retries)).await;
}
