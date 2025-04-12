#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== withdraw ==========================

// withdraw

impl CheckArgs for TokenWithdrawArgs {
    type Result = (SelfCanister, Caller, TokenInfo);
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // ! must be token, can not be dummy lp token
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
async fn token_withdraw(args: TokenWithdrawArgs, retries: Option<u8>) -> TokenChangedResult {
    inner_token_withdraw(args, retries).await.into()
}
#[inline]
async fn inner_token_withdraw(
    args: TokenWithdrawArgs,
    retries: Option<u8>,
) -> Result<candid::Nat, BusinessError> {
    // 1. check args
    let (self_canister, _caller, token) = args.check_args()?;
    let args_clone = args.clone();

    // 2. some value
    let fee_to = vec![];
    let token_account = TokenAccount::new(args.token, args.from);
    let required = vec![token_account];

    // 3. lock
    let lock =
        match super::super::lock_token_balances(fee_to, required, retries.unwrap_or_default())? {
            Lock(guard) => guard,
            Retry(retries) => {
                // ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
                let service_swap = crate::services::swap::Service(self_canister.id());
                return service_swap.token_withdraw(args_clone, Some(retries)).await;
            }
        };

    // * 4. do business
    {
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
        let amount = args.amount_without_fee + token.fee; // Total withdrawal
        with_mut_state_without_record(|s| {
            s.business_token_withdraw(&lock, args.token, args.from, amount)
        })?;

        // ? 3. log
        // ! push log

        Ok(height)
    }
}
