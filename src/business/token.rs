#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== query token and balance ==========================

// anyone can query
#[ic_cdk::query]
fn tokens_query() -> Vec<TokenInfo> {
    with_state(|s| {
        s.business_tokens_query()
            .values()
            .cloned()
            .chain(s.business_dummy_tokens_query().into_values())
            .collect()
    })
}

// anyone can query
#[ic_cdk::query]
fn token_query(token: CanisterId) -> Option<TokenInfo> {
    with_state(|s| {
        if let Some(token_info) = s.business_tokens_query().get(&token) {
            return Some(token_info.clone());
        }
        if let Some(token_info) = s.business_dummy_tokens_query().remove(&token) {
            return Some(token_info.clone());
        }
        None
    })
}

// anyone can query
#[ic_cdk::query]
fn token_balance_of(token: CanisterId, account: Account) -> candid::Nat {
    crate::utils::owner::check_owner_for_token_balance_of(&account.owner); // ! must be owner or self canister
    token_balance_by(token, account)
}

// anyone can query
#[ic_cdk::query]
fn tokens_balance_of(account: Account) -> Vec<(CanisterId, candid::Nat)> {
    crate::utils::owner::check_owner_for_token_balance_of(&account.owner); // ! must be owner or self canister
    tokens_balance_by(account)
}

// anyone can query
#[ic_cdk::query(guard = "has_business_token_balance_by")]
fn token_balance_by(token: CanisterId, account: Account) -> candid::Nat {
    with_state(|s| s.business_token_balance_of(token, account))
}

// anyone can query
#[ic_cdk::query(guard = "has_business_token_balance_by")]
fn tokens_balance_by(account: Account) -> Vec<(CanisterId, candid::Nat)> {
    with_state(|s| {
        let tokens = s.business_tokens_query();
        let dummy_tokens = s.business_dummy_tokens_query();
        tokens
            .keys()
            .chain(dummy_tokens.keys())
            .map(|&canister_id| {
                (
                    canister_id,
                    s.business_token_balance_of(canister_id, account),
                )
            })
            .collect()
    })
}

// ========================== deposit and withdraw ==========================

// deposit
impl CheckArgs for TokenDepositArgs {
    type Result = (SelfCanister, Caller);
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check supported token
        if !with_state(|s| s.business_tokens_query().contains_key(&self.token)) {
            return Err(BusinessError::NotSupportedToken(self.token));
        }

        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        Ok((self_canister, caller))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_deposit")]
async fn token_deposit(args: TokenDepositArgs, retries: Option<u8>) -> TokenTransferResult {
    inner_token_deposit(args, retries).await.into()
}
async fn inner_token_deposit(
    args: TokenDepositArgs,
    retries: Option<u8>,
) -> Result<candid::Nat, BusinessError> {
    // 1. check args
    let (self_canister, _caller) = args.check_args()?;
    let args_clone = args.clone();

    // 2. some value
    let token_account = TokenAccount::new(args.token, args.from);
    let token_accounts = vec![token_account];

    super::with_token_balance_lock(
        &token_accounts,
        retries.unwrap_or_default(),
        || async {
            let service_icrc2 = crate::services::icrc2::Service(args.token);

            // ? 1. transfer token to self
            let self_account = Account {
                owner: self_canister.id(),
                subaccount: None,
            };
            let height = service_icrc2
                .icrc_2_transfer_from(crate::services::icrc2::TransferFromArgs {
                    from: args.from,
                    spender_subaccount: None, // approve subaccount
                    to: self_account,         // * to self
                    amount: args.amount_without_fee.clone(),
                    fee: None, // deposit action doesn't care fee
                    memo: None,
                    created_at_time: None,
                })
                .await
                .map_err(BusinessError::CallCanisterError)?
                .0
                .map_err(BusinessError::TransferFromError)?;

            // ? 2. record changed
            let amount = args.amount_without_fee;
            with_mut_state_without_record(|s| {
                s.business_token_deposit(args.token, args.from, amount);
            });

            // ? 3. log
            // ! push log

            Ok(height)
        },
        // ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
        |retries| async move {
            let service_swap = crate::services::swap::Service(self_canister.id());
            service_swap.token_deposit(args_clone, Some(retries)).await
        },
        |accounts| Err(BusinessError::Locked(accounts)),
    )
    .await
}

// withdraw

impl CheckArgs for TokenWithdrawArgs {
    type Result = (SelfCanister, Caller, TokenInfo);
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        let token = with_state(|s| s.business_tokens_query().get(&self.token).cloned())
            .ok_or(BusinessError::NotSupportedToken(self.token))?;

        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check balance
        let balance = with_state(|s| s.business_token_balance_of(self.token, self.from));
        let amount = self.amount_without_fee.clone() + token.fee.clone();
        if balance < amount {
            return Err(BusinessError::InsufficientBalance((self.token, balance)));
        }

        Ok((self_canister, caller, token))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_withdraw")]
async fn token_withdraw(args: TokenWithdrawArgs, retries: Option<u8>) -> TokenTransferResult {
    inner_token_withdraw(args, retries).await.into()
}
async fn inner_token_withdraw(
    args: TokenWithdrawArgs,
    retries: Option<u8>,
) -> Result<candid::Nat, BusinessError> {
    // 1. check args
    let (self_canister, _caller, token) = args.check_args()?;
    let args_clone = args.clone();

    // 2. some value
    let token_account = TokenAccount::new(args.token, args.from);
    let token_accounts = vec![token_account];

    super::with_token_balance_lock(
        &token_accounts,
        retries.unwrap_or_default(),
        || async {
            let service_icrc2 = crate::services::icrc2::Service(args.token);

            // ? 1. transfer token to user
            let height = service_icrc2
                .icrc_1_transfer(crate::services::icrc2::TransferArg {
                    from_subaccount: None,
                    to: args.to,
                    amount: args.amount_without_fee.clone(),
                    fee: Some(token.fee.clone()), // withdraw action should care fee
                    memo: None,
                    created_at_time: None,
                })
                .await
                .map_err(BusinessError::CallCanisterError)?
                .0
                .map_err(BusinessError::TransferError)?;

            // ? 2. record changed
            let amount = args.amount_without_fee + token.fee;
            with_mut_state_without_record(|s| {
                s.business_token_withdraw(args.token, args.from, amount);
            });

            // ? 3. log
            // ! push log

            Ok(height)
        },
        // ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
        |retries| async move {
            let service_swap = crate::services::swap::Service(self_canister.id());
            service_swap.token_withdraw(args_clone, Some(retries)).await
        },
        |accounts| Err(BusinessError::Locked(accounts)),
    )
    .await
}
