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
    with_state(|s| s.business_tokens_query().values().cloned().collect())
}

// anyone can query
#[ic_cdk::query]
fn token_query(canister_id: CanisterId) -> Option<TokenInfo> {
    with_state(|s| s.business_tokens_query().get(&canister_id).cloned())
}

// anyone can query
#[ic_cdk::query]
fn token_balance_of(canister_id: CanisterId, account: Account) -> candid::Nat {
    with_state(|s| s.business_token_balance_of(canister_id, account))
}

// anyone can query
#[ic_cdk::query]
fn tokens_balance_of(account: Account) -> Vec<(CanisterId, candid::Nat)> {
    with_state(|s| {
        s.business_tokens_query()
            .keys()
            .cloned()
            .map(|canister_id| {
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
        if !with_state(|s| s.business_tokens_query().contains_key(&self.canister_id)) {
            return Err(BusinessError::NotSupportedToken(self.canister_id));
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
    let token_account = TokenAccount::new(args.canister_id, args.from);
    let token_accounts = vec![token_account];

    super::with_token_balance_lock(
        &token_accounts,
        retries.unwrap_or_default(),
        || async {
            let service_icrc2 = crate::services::icrc2::Service(args.canister_id);
            let height = service_icrc2
                .icrc_2_transfer_from(crate::services::icrc2::TransferFromArgs {
                    from: args.from,
                    spender_subaccount: None, // approve subaccount
                    to: Account {
                        owner: self_canister.id(),
                        subaccount: None,
                    },
                    amount: args.amount_without_fee.clone(),
                    fee: None, // deposit action doesn't care fee
                    memo: None,
                    created_at_time: None,
                })
                .await
                .map_err(BusinessError::CallCanisterError)?
                .0
                .map_err(BusinessError::TransferFromError)?;

            let amount = args.amount_without_fee;
            with_mut_state_without_record(|s| {
                s.business_token_deposit(args.canister_id, args.from, amount);
            });

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
        let token = with_state(|s| s.business_tokens_query().get(&self.canister_id).cloned())
            .ok_or(BusinessError::NotSupportedToken(self.canister_id))?;

        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check balance
        let balance = with_state(|s| s.business_token_balance_of(self.canister_id, self.from));
        let amount = self.amount_without_fee.clone() + token.fee.clone();
        if balance < amount {
            return Err(BusinessError::InsufficientBalance((
                self.canister_id,
                balance,
            )));
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
    let token_account = TokenAccount::new(args.canister_id, args.from);
    let token_accounts = vec![token_account];

    super::with_token_balance_lock(
        &token_accounts,
        retries.unwrap_or_default(),
        || async {
            let service_icrc2 = crate::services::icrc2::Service(args.canister_id);

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

            let amount = args.amount_without_fee + token.fee;
            with_mut_state_without_record(|s| {
                s.business_token_withdraw(args.canister_id, args.from, amount);
            });

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
