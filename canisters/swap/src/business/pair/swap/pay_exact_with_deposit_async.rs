#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== pay exact ==========================

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_swap")]
async fn pair_swap_with_deposit_and_async_withdraw(
    args: TokenPairSwapWithDepositAndWithdrawArgs,
) -> (
    TokenChangedResult,
    Option<TokenPairSwapTokensResult>,
    Option<TokenChangedResult>,
) {
    ic_cdk::println!("pair_swap_with_deposit_and_async_withdraw #0: {:?}", args);

    // 1. check args
    let (deposit, swap, token) = match args.check_args() {
        Ok(values) => values,
        Err(err) => return (Err(err).into(), None, None),
    };

    ic_cdk::println!("pair_swap_with_deposit_and_async_withdraw #1: {:?}", deposit);

    // 2. do deposit
    let deposit_result = super::super::super::token::deposit::inner_token_deposit(deposit, Some(3), false).await;
    if deposit_result.is_err() {
        return (deposit_result.into(), None, None);
    }

    ic_cdk::println!(
        "pair_swap_with_deposit_and_async_withdraw #2: {:?} {:?}",
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
        "pair_swap_with_deposit_and_async_withdraw #3: {:?} {:?}",
        swap_result,
        withdraw
    );

    ic_cdk::spawn(async move {
        let withdraw_result =
            super::super::super::token::withdraw::inner_token_withdraw(withdraw, Some(3), false).await;
        ic_cdk::println!("pair_swap_with_deposit_and_async_withdraw #4: {:?}", withdraw_result);
    });

    // Asynchronously triggers synchronization tasks
    crate::business::config::push::inner_push_blocks(true, true);

    (
        deposit_result.into(),
        Some(swap_result.into()),
        // Some(withdraw_result.into()),
        None,
    )
}
