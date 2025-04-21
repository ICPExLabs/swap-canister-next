#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== pay exact ==========================

#[allow(clippy::type_complexity)]
#[inline]
pub(super) fn check_pay_exact(
    args: &TokenPairSwapExactTokensForTokensArgs,
    balance_in: Option<Nat>,
) -> Result<
    (
        TimestampNanos,
        Vec<CanisterId>,
        Vec<TokenAccount>,
        SelfCanister,
        Caller,
        TokenPairSwapExactTokensForTokensArg,
        (Vec<Nat>, Vec<Account>),
    ),
    BusinessError,
> {
    // check owner
    let (self_canister, caller) = check_caller(&args.from.owner)?;

    // check pools
    let mut pas = vec![];
    let mut fee_tokens = vec![];
    let mut required = vec![];
    for pool in &args.path {
        let (pa, _fee_tokens, _required) = check_pool(pool, &self_canister, None)?;
        pas.push(pa);
        fee_tokens.extend(_fee_tokens);
        required.extend(_required);
    }

    let arg = TokenPairSwapExactTokensForTokensArg {
        self_canister,
        pas,
        from: args.from,
        amount_in: args.amount_in.clone(),
        amount_out_min: args.amount_out_min.clone(),
        path: args.path.clone(),
        to: args.to,
    };

    // check path
    check_path(&arg.path)?;

    // check balance in
    let balance_in = balance_in.unwrap_or_else(|| {
        with_state(|s| s.business_token_balance_of(args.path[0].token.0, args.from))
    });
    if balance_in < args.amount_in {
        return Err(BusinessError::insufficient_balance(
            args.path[0].token.0,
            balance_in,
        ));
    }

    // check deadline
    if let Some(deadline) = &args.deadline {
        deadline.check_args()?;
    }

    // check meta
    let now = check_meta(&args.memo, &args.created)?;

    // check arg again
    let checking = with_state(|s| s.business_token_pair_swap_fixed_in_checking(&arg))?;

    Ok((
        now,
        fee_tokens,
        required,
        self_canister,
        caller,
        arg,
        checking,
    ))
}

// pay extra tokens
impl CheckArgs for TokenPairSwapExactTokensForTokensArgs {
    type Result = (
        TimestampNanos,
        Vec<CanisterId>,
        Vec<TokenAccount>,
        SelfCanister,
        Caller,
        TokenPairSwapExactTokensForTokensArg,
        (Vec<Nat>, Vec<Account>),
    );
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        check_pay_exact(self, None)
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
pub async fn inner_pair_swap_exact_tokens_for_tokens(
    args: TokenPairSwapExactTokensForTokensArgs,
    retries: Option<u8>,
) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    // 1. check args
    let (now, fee_tokens, mut required, self_canister, caller, arg, _checking) =
        args.check_args()?;

    // 2. some value
    // let fee_tokens = vec![];
    let token_account_in = TokenAccount::new(args.path[0].token.0, args.from);
    let token_account_out = TokenAccount::new(args.path[args.path.len() - 1].token.1, args.to);
    required.push(token_account_in);
    required.push(token_account_out);

    let success = {
        // 3. lock
        let locks =
            match super::super::super::lock_token_block_chain_and_swap_block_chain_and_token_balances(
                fee_tokens,
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
            with_mut_state(|s| {
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

    // Asynchronously triggers synchronization tasks
    crate::business::config::push::inner_push_blocks(true, true);

    Ok(success)
}
// ! This implicitly contains self_canister_id, which can be called again through permission checks and replaces caller.
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
