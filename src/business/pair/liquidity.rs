#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
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
        Vec<TokenAccount>,
        SelfCanister,
        Caller,
        PairAmm,
        TokenPairLiquidityAddArg,
    );
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check pool
        let (pa, accounts) = check_pool(&self.pool, &self_canister)?;

        let arg = TokenPairLiquidityAddArg {
            from: self.from,
            token_a: self.pool.pair.0,
            token_b: self.pool.pair.1,
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
            return Err(BusinessError::InsufficientBalance((arg.token_a, balance_a)));
        }
        let balance_b = with_state(|s| s.business_token_balance_of(arg.token_b, arg.from));
        if balance_b < arg.amount_b_desired {
            return Err(BusinessError::InsufficientBalance((arg.token_b, balance_b)));
        }

        // check deadline
        if let Some(deadline) = &self.deadline {
            deadline.check_args()?;
        }

        Ok((accounts, self_canister, caller, pa, arg))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_liquidity_add")]
async fn pair_liquidity_add(
    args: TokenPairLiquidityAddArgs,
    retries: Option<u8>,
) -> TokenPairLiquidityAddResult {
    inner_pair_liquidity_add(args, retries).await.into()
}
async fn inner_pair_liquidity_add(
    args: TokenPairLiquidityAddArgs,
    retries: Option<u8>,
) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
    // 1. check args
    let (mut token_accounts, self_canister, _caller, pa, arg) = args.check_args()?;
    let args_clone = args.clone();

    // 2. some value
    let token_account_a = TokenAccount::new(arg.token_a, arg.from);
    let token_account_b = TokenAccount::new(arg.token_b, arg.from);
    token_accounts.push(token_account_a);
    token_accounts.push(token_account_b);

    super::super::with_token_balance_lock(
        &token_accounts,
        retries.unwrap_or_default(),
        || async {
            let success = with_mut_state_without_record(|s| {
                s.business_token_pair_liquidity_add(self_canister, pa, arg)
            })?;

            // ! push log

            Ok(success)
        },
        // ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
        |retries| async move {
            let service_swap = crate::services::swap::Service(self_canister.id());
            service_swap
                .pair_liquidity_add(args_clone, Some(retries))
                .await
        },
        |accounts| Err(BusinessError::Locked(accounts)),
    )
    .await
}

// ========================== remove ==========================

// liquidity remove
impl CheckArgs for TokenPairLiquidityRemoveArgs {
    type Result = (
        Vec<TokenAccount>,
        SelfCanister,
        Caller,
        PairAmm,
        TokenPairLiquidityRemoveArg,
    );
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check pool
        let (pa, accounts) = check_pool(&self.pool, &self_canister)?;

        let arg = TokenPairLiquidityRemoveArg {
            from: self.from,
            token_a: self.pool.pair.0,
            token_b: self.pool.pair.1,
            liquidity: self.liquidity.clone(),
            amount_a_min: self.amount_min.0.clone(),
            amount_b_min: self.amount_min.1.clone(),
            to: self.to,
        };

        // check amount
        arg.check_args()?;

        // check liquidity balance
        with_state(|s| {
            s.business_token_pair_check_liquidity_removable(&pa, &arg.from, &arg.liquidity)
        })?;

        // check deadline
        if let Some(deadline) = &self.deadline {
            deadline.check_args()?;
        }

        Ok((accounts, self_canister, caller, pa, arg))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_liquidity_remove")]
async fn pair_liquidity_remove(
    args: TokenPairLiquidityRemoveArgs,
    retries: Option<u8>,
) -> TokenPairLiquidityRemoveResult {
    inner_pair_liquidity_remove(args, retries).await.into()
}
async fn inner_pair_liquidity_remove(
    args: TokenPairLiquidityRemoveArgs,
    retries: Option<u8>,
) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
    // 1. check args
    let (mut token_accounts, self_canister, _caller, pa, arg) = args.check_args()?;
    let args_clone = args.clone();

    // 2. some value
    let token_account_a = TokenAccount::new(arg.token_a, arg.from);
    let token_account_b = TokenAccount::new(arg.token_b, arg.from);
    token_accounts.push(token_account_a);
    token_accounts.push(token_account_b);

    super::super::with_token_balance_lock(
        &token_accounts,
        retries.unwrap_or_default(),
        || async {
            let success = with_mut_state_without_record(|s| {
                s.business_token_pair_liquidity_remove(self_canister, pa, arg)
            })?;

            // ! push log

            Ok(success)
        },
        // ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
        |retries| async move {
            let service_swap = crate::services::swap::Service(self_canister.id());
            service_swap
                .pair_liquidity_remove(args_clone, Some(retries))
                .await
        },
        |accounts| Err(BusinessError::Locked(accounts)),
    )
    .await
}
