#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== pay exact ==========================

// pay extra tokens
impl CheckArgs for TokenPairSwapWithDepositAndWithdrawArgs {
    type Result = (
        TokenDepositArgs,
        TokenPairSwapExactTokensForTokensArgs,
        TokenInfo,
    );
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check owner
        check_caller(&self.from.owner)?;

        // check path
        check_path(&self.path)?;

        // check args
        let deposit_args: TokenDepositArgs = self.into();
        deposit_args.check_args()?;

        let swap_args: TokenPairSwapExactTokensForTokensArgs = self.into();
        let (.., checking) = swap_args.check_args()?;
        let got = checking.0[checking.0.len() - 1].clone();

        // get token fee
        let token = self.path[self.path.len() - 1].token.1;
        // ! must be token, can not be dummy lp token
        let token = with_state(|s| s.business_tokens_query().get(&token).cloned())
            .ok_or(BusinessError::NotSupportedToken(token))?;
        ic_cdk::println!(
            "token:[{}], fee:{}",
            token.canister_id.to_string(),
            token.fee.to_string()
        );
        if got < token.fee {
            return Err(BusinessError::insufficient_balance(token.canister_id, got));
        }

        Ok((deposit_args, swap_args, token))
    }
}

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_swap")]
async fn pair_swap_with_deposit_and_withdraw(
    args: TokenPairSwapWithDepositAndWithdrawArgs,
) -> (
    TokenChangedResult,
    Option<TokenPairSwapTokensResult>,
    Option<TokenChangedResult>,
) {
    // 1. check args
    let (deposit, swap, token) = match args.check_args() {
        Ok(values) => values,
        Err(err) => return (Err(err).into(), None, None),
    };

    // 2. do deposit
    let deposit_result =
        super::super::super::token::deposit::inner_token_deposit(deposit, Some(3)).await;
    if deposit_result.is_err() {
        return (deposit_result.into(), None, None);
    }

    // 3. do swap
    let swap_result =
        super::pay_exact::inner_pair_swap_exact_tokens_for_tokens(swap, Some(3)).await;
    let got = match &swap_result {
        Ok(success) => success.amounts[success.amounts.len() - 1].clone(),
        Err(_) => return (deposit_result.into(), Some(swap_result.into()), None),
    };
    if got < token.fee {
        return (
            deposit_result.into(),
            Some(swap_result.into()),
            Some(Err(BusinessError::insufficient_balance(token.canister_id, got)).into()),
        );
    }

    // 4. do withdraw
    let withdraw = args.to_withdraw_args(token.canister_id, got - token.fee);
    ic_cdk::println!("withdraw: {:?}", withdraw);
    let withdraw_result =
        super::super::super::token::withdraw::inner_token_withdraw(withdraw, Some(3)).await;

    (
        deposit_result.into(),
        Some(swap_result.into()),
        Some(withdraw_result.into()),
    )
}
