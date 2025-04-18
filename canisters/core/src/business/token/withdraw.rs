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
    type Result = (TimestampNanos, SelfCanister, Caller, TokenInfo);
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // ! must be token, can not be dummy lp token
        let token = with_state(|s| s.business_tokens_query().get(&self.token).cloned())
            .ok_or(BusinessError::NotSupportedToken(self.token))?;

        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check to
        assert!(
            self.to.owner != self_canister.id(),
            "to account can not be swap canister"
        );

        // check fee
        if let Some(fee) = &self.fee {
            if *fee != *crate::utils::math::ZERO && *fee != token.fee {
                return Err(BusinessError::BadTransferFee {
                    expected_fee: token.fee,
                });
            }
        }

        // check balance
        let balance = with_state(|s| s.business_token_balance_of(self.token, self.from));
        let amount = self.withdraw_amount_without_fee.clone() + token.fee.clone();
        if balance < amount {
            return Err(BusinessError::insufficient_balance(self.token, balance));
        }

        // check meta
        let now = check_meta(&self.memo, &self.created)?;

        Ok((now, self_canister, caller, token))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_withdraw")]
async fn token_withdraw(args: TokenWithdrawArgs, retries: Option<u8>) -> TokenChangedResult {
    inner_token_withdraw(args, retries).await.into()
}
#[inline]
pub async fn inner_token_withdraw(
    args: TokenWithdrawArgs,
    retries: Option<u8>,
) -> Result<candid::Nat, BusinessError> {
    // 1. check args
    let (now, self_canister, caller, token) = args.check_args()?;

    // 2. some value
    let fee_to = vec![];
    let token_account_from = TokenAccount::new(args.token, args.from);
    let required = vec![token_account_from];

    let height = {
        // 3. lock
        let locks = match super::super::lock_token_balances_and_token_block_chain(
            fee_to,
            required,
            retries.unwrap_or_default(),
        )? {
            LockResult::Locked(locks) => locks,
            LockResult::Retry(retries) => {
                return retry_token_withdraw(self_canister.id(), args, retries).await;
            }
        };

        // * 4. do business
        {
            let service_icrc2 = crate::services::icrc2::Service(args.token);

            // ? 1. transfer token to user
            let fee = args.fee.unwrap_or(token.fee.clone());
            let transfer_arg = crate::services::icrc2::TransferArg {
                    from_subaccount: None,
                    to: args.to,
                    amount: args.withdraw_amount_without_fee.clone(),
                fee: Some(fee.clone()), // withdraw action should care fee
                    memo: None,
                    created_at_time: None,
            };
            ic_cdk::println!(
                "*call canister transfer_arg* `token:[{}], to:({}), amount:{}, fee:{}`",
                args.token.to_string(),
                display_account(&transfer_arg.to),
                transfer_arg.amount.to_string(),
                token.fee.to_string()
            );
            let height = service_icrc2.icrc_1_transfer(transfer_arg).await?.0?;

            // ? 2. record changed
            let amount = args.withdraw_amount_without_fee + fee; // Total withdrawal
            with_mut_state_without_record(|s| {
                s.business_token_withdraw(
                    &locks,
                    ArgWithMeta {
                        now,
                        caller,
                        arg: WithdrawToken {
                            token: args.token,
                            from: args.from,
                            amount,
                            to: args.to,
                        },
                        memo: args.memo,
                        created: args.created,
                    },
                    height,
                )
            })?
        }
    };

    // TODO 异步触发同步任务

    Ok(height)
}
// ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
#[inline]
async fn retry_token_withdraw(
    self_canister_id: CanisterId,
    args: TokenWithdrawArgs,
    retries: u8,
) -> Result<candid::Nat, BusinessError> {
    let service_swap = crate::services::swap::Service(self_canister_id);
    return service_swap.token_withdraw(args, Some(retries)).await;
}
