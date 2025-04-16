#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== swap ==========================

// pay extra tokens
impl CheckArgs for TokenPairSwapExactTokensForTokensArgs {
    type Result = (
        TimestampNanos,
        Vec<CanisterId>,
        Vec<TokenAccount>,
        SelfCanister,
        Caller,
        TokenPairSwapExactTokensForTokensArg,
    );
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check pools
        let mut pas = vec![];
        let mut fee_to = vec![];
        let mut required = vec![];
        for pool in &self.path {
            let (pa, _fee_to, _required) = check_pool(pool, &self_canister, None)?;
            pas.push(pa);
            fee_to.extend(_fee_to);
            required.extend(_required);
        }

        let arg = TokenPairSwapExactTokensForTokensArg {
            self_canister,
            pas,
            from: self.from,
            amount_in: self.amount_in.clone(),
            amount_out_min: self.amount_out_min.clone(),
            path: self.path.clone(),
            to: self.to,
        };

        // check path
        check_path(&arg.path)?;

        // check balance in
        let balance_in =
            with_state(|s| s.business_token_balance_of(self.path[0].token.0, self.from));
        if balance_in < self.amount_in {
            return Err(BusinessError::insufficient_balance(
                self.path[0].token.0,
                balance_in,
            ));
        }

        // check deadline
        if let Some(deadline) = &self.deadline {
            deadline.check_args()?;
        }

        // check meta
        let now = check_meta(&self.memo, &self.created)?;

        Ok((now, fee_to, required, self_canister, caller, arg))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_swap")]
async fn pair_swap_exact_tokens_for_tokens(
    args: TokenPairSwapExactTokensForTokensArgs,
    retries: Option<u8>,
) -> TokenPairSwapTokensResult {
    inner_pair_swap_exact_tokens_for_tokens(args, retries)
        .await
        .into()
}
#[inline]
async fn inner_pair_swap_exact_tokens_for_tokens(
    args: TokenPairSwapExactTokensForTokensArgs,
    retries: Option<u8>,
) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    // 1. check args
    let (now, fee_to, mut required, self_canister, caller, arg) = args.check_args()?;

    // 2. some value
    // let fee_to = fee_to;
    let token_account_in = TokenAccount::new(args.path[0].token.0, args.from);
    let token_account_out = TokenAccount::new(args.path[args.path.len() - 1].token.1, args.to);
    required.push(token_account_in);
    required.push(token_account_out);

    let success = {
        // 3. lock
        let locks =
            match super::super::lock_token_balances_and_token_block_chain_and_swap_block_chain(
                fee_to,
                required,
                retries.unwrap_or_default(),
            )? {
                LockResult::Locked(locks) => locks,
                LockResult::Retry(retries) => {
                    return retry_pair_swap_exact_tokens_for_tokens(
                        self_canister.id(),
                        args,
                        retries,
                    )
                    .await;
                }
            };

        // * 4. do business
        {
            with_mut_state_without_record(|s| {
                s.business_token_pair_swap_exact_tokens_for_tokens(
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

    // TODO 异步触发同步任务

    Ok(success)
}
// ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
#[inline]
async fn retry_pair_swap_exact_tokens_for_tokens(
    self_canister_id: CanisterId,
    args: TokenPairSwapExactTokensForTokensArgs,
    retries: u8,
) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    let service_swap = crate::services::swap::Service(self_canister_id);
    return service_swap
        .pair_swap_exact_tokens_for_tokens(args, Some(retries))
        .await;
}

// got extra tokens
impl CheckArgs for TokenPairSwapTokensForExactTokensArgs {
    type Result = (
        TimestampNanos,
        Vec<CanisterId>,
        Vec<TokenAccount>,
        SelfCanister,
        Caller,
        TokenPairSwapTokensForExactTokensArg,
    );
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check pools
        let mut pas = vec![];
        let mut fee_to = vec![];
        let mut required = vec![];
        for pool in &self.path {
            let (pa, _fee_to, _required) = check_pool(pool, &self_canister, None)?;
            pas.push(pa);
            fee_to.extend(_fee_to);
            required.extend(_required);
        }

        let arg = TokenPairSwapTokensForExactTokensArg {
            self_canister,
            pas,
            from: self.from,
            amount_out: self.amount_out.clone(),
            amount_in_max: self.amount_in_max.clone(),
            path: self.path.clone(),
            to: self.to,
        };

        // check path
        check_path(&arg.path)?;

        // check balance in // 内部计算出最小输入后，再检查

        // check deadline
        if let Some(deadline) = &self.deadline {
            deadline.check_args()?;
        }

        // check meta
        let now = check_meta(&self.memo, &self.created)?;

        Ok((now, fee_to, required, self_canister, caller, arg))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_swap")]
async fn pair_swap_tokens_for_exact_tokens(
    args: TokenPairSwapTokensForExactTokensArgs,
    retries: Option<u8>,
) -> TokenPairSwapTokensResult {
    inner_pair_swap_tokens_for_exact_tokens(args, retries)
        .await
        .into()
}
#[inline]
async fn inner_pair_swap_tokens_for_exact_tokens(
    args: TokenPairSwapTokensForExactTokensArgs,
    retries: Option<u8>,
) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    // 1. check args
    let (now, fee_to, mut required, self_canister, caller, arg) = args.check_args()?;

    // 2. some value
    // let fee_to = fee_to;
    let token_account_in = TokenAccount::new(args.path[0].token.0, args.from);
    let token_account_out = TokenAccount::new(args.path[args.path.len() - 1].token.1, args.to);
    required.push(token_account_in);
    required.push(token_account_out);

    let success = {
        // 3. lock
        let locks =
            match super::super::lock_token_balances_and_token_block_chain_and_swap_block_chain(
                fee_to,
                required,
                retries.unwrap_or_default(),
            )? {
                LockResult::Locked(locks) => locks,
                LockResult::Retry(retries) => {
                    return retry_pair_swap_tokens_for_exact_tokens(
                        self_canister.id(),
                        args,
                        retries,
                    )
                    .await;
                }
            };

        // * 4. do business
        {
            with_mut_state_without_record(|s| {
                s.business_token_pair_swap_tokens_for_exact_tokens(
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

    // TODO 异步触发同步任务

    Ok(success)
}
// ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
#[inline]
async fn retry_pair_swap_tokens_for_exact_tokens(
    self_canister_id: CanisterId,
    args: TokenPairSwapTokensForExactTokensArgs,
    retries: u8,
) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    let service_swap = crate::services::swap::Service(self_canister_id);
    return service_swap
        .pair_swap_tokens_for_exact_tokens(args, Some(retries))
        .await;
}
