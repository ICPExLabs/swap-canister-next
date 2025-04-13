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
    type Result = (TimestampNanos, SelfCanister, Caller, TokenInfo);
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

        // check meta
        let now = check_meta(&self.memo, &self.created)?;

        Ok((now, self_canister, caller, token))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_transfer")]
async fn token_transfer(args: TokenTransferArgs, retries: Option<u8>) -> TokenChangedResult {
    inner_token_transfer(args, retries).await.into()
}
#[inline]
async fn inner_token_transfer(
    args: TokenTransferArgs,
    retries: Option<u8>,
) -> Result<candid::Nat, BusinessError> {
    // 1. check args
    let (now, self_canister, caller, token) = args.check_args()?;

    // 2. some value
    let fee_to = vec![args.token]; // ! There is a handling fee for this operation
    let token_account_from = TokenAccount::new(args.token, args.from);
    let token_account_to = TokenAccount::new(args.token, args.to);
    let required = vec![token_account_from, token_account_to];

    let changed = {
        // 3. lock
        let locks = match super::super::lock_token_balances_and_token_block_chain(
            fee_to,
            required,
            retries.unwrap_or_default(),
        )? {
            LockResult::Locked(locks) => locks,
            LockResult::Retry(retries) => {
                return retry_token_transfer(self_canister.id(), args, retries).await;
            }
        };

        // * 4. do business
        {
            // ? 0. get transfer fee
            let fee_to = locks.0.fee_to();
            let fee_to = fee_to
                .iter()
                .find(|&fee_to| fee_to.token == args.token)
                .cloned()
                .map(|token_account| token_account.account);

            // ? 1. transfer
            let changed = with_mut_state_without_record(|s| {
                s.business_token_transfer(
                    &locks,
                    ArgWithMeta {
                        now,
                        caller,
                        arg: TransferToken {
                            token: args.token,
                            from: args.from,
                            amount: args.amount_without_fee,
                            to: args.to,
                            fee: fee_to.map(|fee_to| TransferFee {
                                fee: token.fee,
                                fee_to,
                            }),
                        },
                        memo: args.memo,
                        created: args.created,
                    },
                )
            })?;

            changed
        }
    };

    // TODO 异步触发同步任务

    Ok(changed)
}
// ! 这里隐式包含 self_canister_id 能通过权限检查, 替 caller 进行再次调用
#[inline]
async fn retry_token_transfer(
    self_canister_id: CanisterId,
    args: TokenTransferArgs,
    retries: u8,
) -> Result<candid::Nat, BusinessError> {
    let service_swap = crate::services::swap::Service(self_canister_id);
    return service_swap.token_transfer(args, Some(retries)).await;
}
