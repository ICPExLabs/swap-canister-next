#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== swap ==========================

// pay loan tokens
impl CheckArgs for TokenPairSwapByLoanArgs {
    type Result = (
        TimestampNanos,
        Vec<CanisterId>,
        Vec<TokenAccount>,
        SelfCanister,
        Caller,
        TokenPairSwapByLoanArg,
    );
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // ! refuse all action about frozen token
        for p in &self.path {
            with_state(|s| s.business_token_alive(&p.token.0))?;
            with_state(|s| s.business_token_alive(&p.token.1))?;
        }

        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check pools
        let mut pas = vec![];
        let mut fee_tokens = vec![];
        let mut required = vec![];
        for pool in &self.path {
            let (pa, _fee_tokens, _required) = check_pool(pool, &self_canister, None)?;
            pas.push(pa);
            fee_tokens.extend(_fee_tokens);
            required.extend(_required);
        }

        let arg = TokenPairSwapByLoanArg {
            self_canister,
            pas,
            from: self.from,
            loan: self.loan.clone(),
            path: self.path.clone(),
            to: self.to,
        };

        // check path
        check_path(&arg.path)?;

        // ! Check whether the token is consistent
        if self.path[0].token.0 != self.path[self.path.len() - 1].token.1 {
            return Err(BusinessError::Swap("INVALID_PATH".into()));
        }

        // check deadline
        if let Some(deadline) = &self.deadline {
            deadline.check_args()?;
        }

        // check meta
        let now = check_meta(&self.memo, &self.created)?;

        // check arg again
        with_state(|s| {
            s.business_token_pair_swap_fixed_in_checking(&TokenPairSwapExactTokensForTokensArg {
                self_canister,
                pas: arg.pas.clone(),
                from: arg.to,
                amount_in: arg.loan.clone(),
                amount_out_min: arg.loan.clone(),
                path: arg.path.clone(),
                to: arg.to,
            })
        })?;

        Ok((now, fee_tokens, required, self_canister, caller, arg))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_swap")]
async fn pair_swap_by_loan(args: TokenPairSwapByLoanArgs, retries: Option<u8>) -> TokenPairSwapTokensResult {
    inner_pair_swap_by_loan(args, retries).await.into()
}
#[inline]
async fn inner_pair_swap_by_loan(
    args: TokenPairSwapByLoanArgs,
    retries: Option<u8>,
) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    // 1. check args
    let (now, fee_tokens, mut required, self_canister, caller, arg) = args.check_args()?;

    // 2. some value
    // let fee_tokens = vec![];
    let token_account_out = TokenAccount::new(args.path[args.path.len() - 1].token.1, args.to);
    required.push(token_account_out);

    let success = {
        // 3. lock
        let locks =
            match super::super::super::lock_token_block_chain_and_swap_block_chain_and_token_balances_and_token_pairs(
                fee_tokens,
                required,
                arg.pas.clone(),
                retries.unwrap_or_default(),
            )? {
                LockResult::Locked(locks) => locks,
                LockResult::Retry(retries) => {
                    return retry_pair_swap_by_loan(self_canister.id(), args, retries).await;
                }
            };

        // * 4. do business
        {
            with_mut_state(|s| {
                s.business_token_pair_swap_by_loan(
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
async fn retry_pair_swap_by_loan(
    self_canister_id: CanisterId,
    args: TokenPairSwapByLoanArgs,
    retries: u8,
) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    ic_cdk::println!("🔄 retry_pair_swap_by_loan: {}", retries);
    let service_swap = crate::services::swap::Service(self_canister_id);
    return service_swap.pair_swap_by_loan(args, Some(retries)).await;
}
