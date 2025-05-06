#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== pay exact ==========================

// pay extra tokens
impl CheckArgs for TokenPairSwapWithDepositAndWithdrawArgs {
    type Result = (TokenDepositArgs, TokenPairSwapExactTokensForTokensArgs, TokenInfo);
    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // ! refuse all action about frozen token
        for p in &self.path {
            with_state(|s| s.business_token_alive(&p.token.0))?;
            with_state(|s| s.business_token_alive(&p.token.1))?;
        }

        // check owner
        check_caller(&self.from.owner)?;

        // check path
        check_path(&self.path)?;

        // check args
        let deposit_args: TokenDepositArgs = self.into();
        deposit_args.check_args()?;

        let swap_args: TokenPairSwapExactTokensForTokensArgs = self.into();
        let (.., (amounts, _)) =
            super::pay_exact::check_pay_exact(&swap_args, Some(deposit_args.deposit_amount_without_fee.clone()))?;
        let got = amounts[amounts.len() - 1].clone();

        // get token fee
        let token = self.path[self.path.len() - 1].token.1;
        // ! must be token, can not be dummy lp token
        let token = with_state(|s| s.business_tokens_query().get(&token).map(|t| t.clone().into_owned()))
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
    inner_pair_swap_with_deposit_and_withdraw(args, false).await
}
// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_swap")]
async fn pair_swap_with_deposit_and_withdraw_async(
    args: TokenPairSwapWithDepositAndWithdrawArgs,
) -> (
    TokenChangedResult,
    Option<TokenPairSwapTokensResult>,
    Option<TokenChangedResult>,
) {
    inner_pair_swap_with_deposit_and_withdraw(args, true).await
}

async fn inner_pair_swap_with_deposit_and_withdraw(
    args: TokenPairSwapWithDepositAndWithdrawArgs,
    _async: bool,
) -> (
    TokenChangedResult,
    Option<TokenPairSwapTokensResult>,
    Option<TokenChangedResult>,
) {
    ic_cdk::println!("pair_swap_with_deposit_and_withdraw(async:{_async}) #0: {:?}", args);

    // 1. check args
    let (deposit, swap, token) = match args.check_args() {
        Ok(values) => values,
        Err(err) => return (Err(err).into(), None, None),
    };

    ic_cdk::println!("pair_swap_with_deposit_and_withdraw(async:{_async}) #1: {:?}", deposit);

    // 2. do deposit
    let deposit_result = super::super::super::token::deposit::inner_token_deposit(deposit, Some(3), false).await;
    if deposit_result.is_err() {
        return (deposit_result.into(), None, None);
    }

    ic_cdk::println!(
        "pair_swap_with_deposit_and_withdraw(async:{_async}) #2: {:?} {:?}",
        deposit_result,
        swap
    );

    // 3. do swap
    let swap_result = super::pay_exact::inner_pair_swap_exact_tokens_for_tokens(swap, Some(3), false).await;
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

    ic_cdk::println!(
        "pair_swap_with_deposit_and_withdraw(async:{_async}) #3: {:?} {:?}",
        swap_result,
        withdraw
    );

    let withdraw_result = if _async {
        ic_cdk::futures::spawn(async move {
            let withdraw_result =
                super::super::super::token::withdraw::inner_token_withdraw(withdraw, Some(3), false).await;
            ic_cdk::println!("pair_swap_with_deposit_and_async_withdraw #4: {:?}", withdraw_result);
        });

        super::super::super::delay_task(|| {
            // Asynchronously triggers synchronization tasks
            crate::business::config::push::inner_push_blocks(true, true);
        });

        None
    } else {
        let withdraw_result =
            super::super::super::token::withdraw::inner_token_withdraw(withdraw, Some(3), false).await;

        ic_cdk::println!(
            "pair_swap_with_deposit_and_withdraw(async:{_async}) #4: {:?}",
            withdraw_result
        );

        // Asynchronously triggers synchronization tasks
        crate::business::config::push::inner_push_blocks(true, true);

        Some(withdraw_result.into())
    };

    (deposit_result.into(), Some(swap_result.into()), withdraw_result)
}
