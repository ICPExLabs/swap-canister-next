#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== remove ==========================

// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_liquidity_remove")]
async fn pair_liquidity_remove_and_withdraw(
    args: TokenPairLiquidityRemoveArgs,
) -> (TokenPairLiquidityRemoveResult, Option<ManyTokenChangedResult>) {
    inner_pair_liquidity_remove_and_withdraw(args, false).await
}
// check forbidden
#[ic_cdk::update(guard = "has_business_token_pair_liquidity_remove")]
async fn pair_liquidity_remove_and_withdraw_async(
    args: TokenPairLiquidityRemoveArgs,
) -> (TokenPairLiquidityRemoveResult, Option<ManyTokenChangedResult>) {
    inner_pair_liquidity_remove_and_withdraw(args, true).await
}

async fn inner_pair_liquidity_remove_and_withdraw(
    args: TokenPairLiquidityRemoveArgs,
    _async: bool,
) -> (TokenPairLiquidityRemoveResult, Option<ManyTokenChangedResult>) {
    // query data
    let token_a = match with_state(|s| s.business_tokens_query().get(&args.swap_pair.token.0).cloned()) {
        Some(token) => token,
        None => {
            return (
                Err(BusinessError::NotSupportedToken(args.swap_pair.token.0)).into(),
                None,
            );
        }
    };
    let token_b = match with_state(|s| s.business_tokens_query().get(&args.swap_pair.token.1).cloned()) {
        Some(token) => token,
        None => {
            return (
                Err(BusinessError::NotSupportedToken(args.swap_pair.token.1)).into(),
                None,
            );
        }
    };
    let withdraw_from = args.to;

    // 1. do remove
    let remove = match super::remove::inner_pair_liquidity_remove(args, None).await {
        Ok(success) => success,
        Err(err) => return (Err(err).into(), None),
    };

    // 2. do withdraw
    let withdraw_args_a = TokenWithdrawArgs {
        token: token_a.canister_id,
        from: withdraw_from,
        withdraw_amount_without_fee: remove.amount.0.clone() - token_a.fee,
        to: withdraw_from,
        fee: None,
        memo: None,
        created: None,
    };
    let withdraw_args_b = TokenWithdrawArgs {
        token: token_a.canister_id,
        from: withdraw_from,
        withdraw_amount_without_fee: remove.amount.1.clone() - token_b.fee,
        to: withdraw_from,
        fee: None,
        memo: None,
        created: None,
    };
    let args = vec![withdraw_args_a, withdraw_args_b];
    let withdraw_many = if _async {
        ic_cdk::spawn(async move {
            let _withdraw_many = super::super::super::token::withdraw::many::inner_token_withdraw_many(
                TokenWithdrawManyArgs { args },
                None,
                false,
            )
            .await;
        });

        super::super::super::delay_task(|| {
            // Asynchronously triggers synchronization tasks
            crate::business::config::push::inner_push_blocks(true, true);
        });

        None
    } else {
        let withdraw_many = super::super::super::token::withdraw::many::inner_token_withdraw_many(
            TokenWithdrawManyArgs { args },
            None,
            false,
        )
        .await;

        // Asynchronously triggers synchronization tasks
        crate::business::config::push::inner_push_blocks(true, true);

        Some(withdraw_many.into())
    };

    (Ok(remove).into(), withdraw_many)
}
