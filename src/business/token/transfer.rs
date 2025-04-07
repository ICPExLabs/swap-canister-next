#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== inner transfer ==========================

// inner transfer
impl CheckArgs for TokenTransferArgs {
    type Result = (SelfCanister, Caller, TokenInfo);
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check supported token, can be token or dummy lp token
        let token = with_state(|s| {
            s.business_all_tokens_query()
                .remove(&self.token)
                .map(|token| token.into_owned())
        })
        .ok_or(BusinessError::NotSupportedToken(self.token))?;

        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check balance
        let balance = with_state(|s| s.business_token_balance_of(token.canister_id, self.from));
        let amount = self.amount_without_fee.clone() + token.fee.clone();
        if balance < amount {
            return Err(BusinessError::InsufficientBalance((
                token.canister_id,
                balance,
            )));
        }

        Ok((self_canister, caller, token))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_transfer")]
async fn token_transfer(args: TokenTransferArgs, retries: Option<u8>) -> TokenChangedResult {
    inner_token_transfer(args, retries).await.into()
}
async fn inner_token_transfer(
    args: TokenTransferArgs,
    retries: Option<u8>,
) -> Result<candid::Nat, BusinessError> {
    // 1. check args
    let (self_canister, _caller, token) = args.check_args()?;
    let args_clone = args.clone();

    // 2. some value
    let token_account_from = TokenAccount::new(args.token, args.from);
    let token_account_to = TokenAccount::new(args.token, args.to);
    let token_accounts = vec![token_account_from, token_account_to];

    super::super::with_token_balance_lock(
        &token_accounts,
        retries.unwrap_or_default(),
        || async {
            // ? 1. transfer
            with_mut_state_without_record(|s| {
                s.business_token_transfer(
                    args.token,
                    args.from,
                    args.to,
                    args.amount_without_fee.clone(),
                    token.fee,
                )
            });

            // ? 2. log
            // ! push log

            Ok(args.amount_without_fee)
        },
        // ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
        |retries| async move {
            let service_swap = crate::services::swap::Service(self_canister.id());
            service_swap.token_transfer(args_clone, Some(retries)).await
        },
        |accounts| Err(BusinessError::Locked(accounts)),
    )
    .await
}
