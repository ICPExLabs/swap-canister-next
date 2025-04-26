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
            s.business_all_tokens_with_dummy_query()
                .remove(&self.token)
                .map(|token| token.into_owned())
        })
        .ok_or(BusinessError::NotSupportedToken(self.token))?;

        // check owner
        let (self_canister, caller) = check_caller(&self.from.owner)?;

        // check to
        assert!(
            self.to.owner != self_canister.id(),
            "to account can not be swap canister"
        );
        assert!(self.from.owner != self.to.owner, "to account can not be from account");

        // check fee
        if let Some(fee) = &self.fee {
            if *fee != token.fee {
                return Err(BusinessError::invalid_transfer_fee(token.canister_id, token.fee));
            }
        }

        // check balance
        let (balance, fee_to) = with_state(|s| s.business_token_balance_of_with_fee_to(token.canister_id, self.from));
        let amount = self.transfer_amount_without_fee.clone() + fee_to.map(|_| token.fee.clone()).unwrap_or_default();
        if balance < amount {
            return Err(BusinessError::insufficient_balance(token.canister_id, balance));
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
async fn inner_token_transfer(args: TokenTransferArgs, retries: Option<u8>) -> Result<candid::Nat, BusinessError> {
    // 1. check args
    let (now, self_canister, caller, token) = args.check_args()?;

    // 2. some value
    let fee_tokens = vec![args.token]; // ! There is a handling fee for this operation
    let token_account_from = TokenAccount::new(args.token, args.from);
    let token_account_to = TokenAccount::new(args.token, args.to);
    let required = vec![token_account_from, token_account_to];

    let changed = if token.is_lp_token {
        // 3. lock
        let locks = match super::super::lock_token_block_chain_and_swap_block_chain_and_token_balances(
            fee_tokens,
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
            let fee_to = locks.0.fee_to; // token fee to

            // ? 1. transfer
            with_mut_state(|s| {
                s.business_token_transfer_lp(
                    &locks,
                    ArgWithMeta {
                        now,
                        caller,
                        arg: TransferToken {
                            token: args.token,
                            from: args.from,
                            amount: args.transfer_amount_without_fee,
                            to: args.to,
                            fee: fee_to.map(|fee_to| TransferFee { fee: token.fee, fee_to }),
                        },
                        memo: args.memo,
                        created: args.created,
                    },
                )
            })?
        }
    } else {
        // 3. lock
        let locks = match super::super::lock_token_block_chain_and_token_balances(
            fee_tokens,
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
            let fee_to = locks.0.fee_to; // token fee to

            // ? 1. transfer
            with_mut_state(|s| {
                s.business_token_transfer(
                    &locks,
                    ArgWithMeta {
                        now,
                        caller,
                        arg: TransferToken {
                            token: args.token,
                            from: args.from,
                            amount: args.transfer_amount_without_fee,
                            to: args.to,
                            fee: fee_to.map(|fee_to| TransferFee { fee: token.fee, fee_to }),
                        },
                        memo: args.memo,
                        created: args.created,
                    },
                )
            })?
        }
    };

    // Asynchronously triggers synchronization tasks
    crate::business::config::push::inner_push_blocks(true, false);

    Ok(changed)
}
// ! This implicitly contains self_canister_id, which can be called again through permission checks and replaces caller.
#[inline]
async fn retry_token_transfer(
    self_canister_id: CanisterId,
    args: TokenTransferArgs,
    retries: u8,
) -> Result<candid::Nat, BusinessError> {
    ic_cdk::println!("🔄 retry_token_transfer: {}", retries);
    let service_swap = crate::services::swap::Service(self_canister_id);
    return service_swap.token_transfer(args, Some(retries)).await;
}
