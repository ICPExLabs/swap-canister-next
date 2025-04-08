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
    type Result = (Vec<TokenAccount>, SelfCanister, Caller, Vec<PairAmm>);
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check path
        if self.path.is_empty() {
            return Err(BusinessError::Swap("INVALID_PATH".into()));
        }
        if 1 < self.path.len() {
            // 循环检查代币是否相连
            let mut i = 1;
            loop {
                if self.path.len() <= i {
                    break;
                }

                let path0 = &self.path[i - 1];
                let path1 = &self.path[i];

                if path0.pair.1 != path1.pair.0 {
                    return Err(BusinessError::Swap("INVALID_PATH".into()));
                }

                i += 1;
            }
        }

        // check pools
        let mut pas = vec![];
        let mut token_accounts = vec![];
        for pool in &self.path {
            let (pa, accounts) = check_pool(pool, &self_canister, None)?;
            pas.push(pa);
            token_accounts.extend(accounts);
        }

        // check balance in
        let balance_in =
            with_state(|s| s.business_token_balance_of(self.path[0].pair.0, self.from));
        if balance_in < self.amount_in {
            return Err(BusinessError::InsufficientBalance((
                self.path[0].pair.0,
                balance_in,
            )));
        }

        // check deadline
        if let Some(deadline) = &self.deadline {
            deadline.check_args()?;
        }

        Ok((token_accounts, self_canister, caller, pas))
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
async fn inner_pair_swap_exact_tokens_for_tokens(
    args: TokenPairSwapExactTokensForTokensArgs,
    retries: Option<u8>,
) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    // 1. check args
    let (mut token_accounts, self_canister, _caller, pas) = args.check_args()?;
    let args_clone = args.clone();

    // 2. some value
    let token_account_in = TokenAccount::new(args.path[0].pair.0, args.from);
    let token_account_out = TokenAccount::new(args.path[args.path.len() - 1].pair.1, args.to);
    token_accounts.push(token_account_in);
    token_accounts.push(token_account_out);

    super::super::with_token_balance_lock(
        &token_accounts,
        retries.unwrap_or_default(),
        || async {
            let success = with_mut_state_without_record(|s| {
                s.business_token_pair_swap_exact_tokens_for_tokens(&self_canister, args, pas)
            })?;

            // ! push log

            Ok(success)
        },
        // ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
        |retries| async move {
            let service_swap = crate::services::swap::Service(self_canister.id());
            service_swap
                .pair_swap_exact_tokens_for_tokens(args_clone, Some(retries))
                .await
        },
        |accounts| Err(BusinessError::Locked(accounts)),
    )
    .await
}

// got extra tokens
impl CheckArgs for TokenPairSwapTokensForExactTokensArgs {
    type Result = (Vec<TokenAccount>, SelfCanister, Caller, Vec<PairAmm>);
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check path
        if self.path.is_empty() {
            return Err(BusinessError::Swap("INVALID_PATH".into()));
        }
        if 1 < self.path.len() {
            // 循环检查代币是否相连
            let mut i = 1;
            loop {
                if self.path.len() <= i {
                    break;
                }

                let path0 = &self.path[i - 1];
                let path1 = &self.path[i];

                if path0.pair.1 != path1.pair.0 {
                    return Err(BusinessError::Swap("INVALID_PATH".into()));
                }

                i += 1;
            }
        }

        // check pools
        let mut pas = vec![];
        let mut token_accounts = vec![];
        for pool in &self.path {
            let (pa, accounts) = check_pool(pool, &self_canister, None)?;
            pas.push(pa);
            token_accounts.extend(accounts);
        }

        // check deadline
        if let Some(deadline) = &self.deadline {
            deadline.check_args()?;
        }

        Ok((token_accounts, self_canister, caller, pas))
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
async fn inner_pair_swap_tokens_for_exact_tokens(
    args: TokenPairSwapTokensForExactTokensArgs,
    retries: Option<u8>,
) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    // 1. check args
    let (mut token_accounts, self_canister, _caller, pas) = args.check_args()?;
    let args_clone = args.clone();

    // 2. some value
    let token_account_in = TokenAccount::new(args.path[0].pair.0, args.from);
    let token_account_out = TokenAccount::new(args.path[args.path.len() - 1].pair.1, args.to);
    token_accounts.push(token_account_in);
    token_accounts.push(token_account_out);

    super::super::with_token_balance_lock(
        &token_accounts,
        retries.unwrap_or_default(),
        || async {
            let success = with_mut_state_without_record(|s| {
                s.business_token_pair_swap_tokens_for_exact_tokens(&self_canister, args, pas)
            })?;

            // ! push log

            Ok(success)
        },
        // ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
        |retries| async move {
            let service_swap = crate::services::swap::Service(self_canister.id());
            service_swap
                .pair_swap_tokens_for_exact_tokens(args_clone, Some(retries))
                .await
        },
        |accounts| Err(BusinessError::Locked(accounts)),
    )
    .await
}
