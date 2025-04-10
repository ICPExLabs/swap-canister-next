#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== deposit ==========================

// deposit
impl CheckArgs for TokenDepositArgs {
    type Result = (SelfCanister, Caller);
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // ! must be token, can not be dummy lp token
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
async fn token_deposit(args: TokenDepositArgs, retries: Option<u8>) -> TokenChangedResult {
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

    super::super::with_token_balance_lock(
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
