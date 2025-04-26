#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== withdraw many ==========================

// withdraw many

impl CheckArgs for TokenWithdrawManyArgs {
    type Result = Vec<(TimestampNanos, SelfCanister, Caller, TokenInfo)>;
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // ! refuse all action about frozen token
        for a in &self.args {
            with_state(|s| s.business_token_alive(&a.token))?;
        }

        let mut args = Vec::with_capacity(self.args.len());

        for a in &self.args {
            args.push(a.check_args()?);
        }

        // check from and to
        let from = self.args.iter().map(|a| a.from).collect::<HashSet<_>>();
        assert_eq!(from.len(), 1, "from account must be same");
        let to = self.args.iter().map(|a| a.to).collect::<HashSet<_>>();
        assert_eq!(to.len(), 1, "to account must be same");

        // check token
        let token = self.args.iter().map(|a| a.token).collect::<HashSet<_>>();
        assert_eq!(token.len(), self.args.len(), "token must be different token");

        Ok(args)
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_withdraw")]
async fn token_withdraw_many(args: TokenWithdrawManyArgs, retries: Option<u8>) -> ManyTokenChangedResult {
    inner_token_withdraw_many(args, retries, true).await.into()
}
#[inline]
pub async fn inner_token_withdraw_many(
    args: TokenWithdrawManyArgs,
    retries: Option<u8>,
    push: bool,
) -> Result<Vec<Result<candid::Nat, BusinessError>>, BusinessError> {
    // 1. check args
    let list = args.check_args()?;

    // 2. some value
    let fee_tokens = vec![];
    let token_account_from = args
        .args
        .iter()
        .map(|a| TokenAccount::new(a.token, a.from))
        .collect::<Vec<_>>();
    let required = token_account_from;

    let list = {
        // 3. lock
        let locks = match super::super::super::lock_token_block_chain_and_token_balances(
            fee_tokens,
            required,
            retries.unwrap_or_default(),
        )? {
            LockResult::Locked(locks) => locks,
            LockResult::Retry(retries) => {
                return retry_token_withdraw_many(list[0].1.id(), args, retries).await;
            }
        };

        let locks = &locks;

        let list = args
            .args
            .into_iter()
            .zip(list.into_iter())
            .map(|(args, (now, _self_canister, caller, token))| async move {
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
                    with_mut_state(|s| {
                        s.business_token_withdraw(
                            locks,
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
                    })
                }
            })
            .collect::<Vec<_>>();

        futures::future::join_all(list).await.into_iter().collect::<Vec<_>>()
    };

    // Asynchronously triggers synchronization tasks
    if push {
        crate::business::config::push::inner_push_blocks(true, false);
    }

    Ok(list)
}
// ! This implicitly contains self_canister_id, which can be called again through permission checks and replaces caller.
#[inline]
async fn retry_token_withdraw_many(
    self_canister_id: CanisterId,
    args: TokenWithdrawManyArgs,
    retries: u8,
) -> Result<Vec<Result<candid::Nat, BusinessError>>, BusinessError> {
    ic_cdk::println!("🔄 retry_token_withdraw_many: {}", retries);
    let service_swap = crate::services::swap::Service(self_canister_id);
    return service_swap.token_withdraw_many(args, Some(retries)).await;
}
