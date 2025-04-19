#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

pub mod archive;

pub mod config;

pub mod token;

pub mod pair;

pub mod test;

#[allow(unused)]
#[inline(always)]
fn lock_token_balances(
    required: Vec<TokenAccount>,
    retries: u8,
) -> Result<LockResult<TokenBalancesLock>, BusinessError> {
    assert!(retries < 10, "Too many retries");

    match with_mut_state(|s| s.business_token_balance_lock(required)) {
        Ok(lock) => Ok(LockResult::Locked(lock)),
        Err(locked) => {
            if 0 < retries {
                Ok(LockResult::Retry(retries - 1))
            } else {
                Err(BusinessError::TokenAccountsLocked(locked))
            }
        }
    }
}

#[allow(unused)]
#[inline(always)]
fn lock_token_block_chain(retries: u8) -> Result<LockResult<TokenBlockChainLock>, BusinessError> {
    assert!(retries < 10, "Too many retries");

    match with_mut_state(|s| s.business_token_block_chain_lock()) {
        Some(lock) => Ok(LockResult::Locked(lock)),
        None => {
            if 0 < retries {
                Ok(LockResult::Retry(retries - 1))
            } else {
                Err(BusinessError::TokenBlockChainLocked)
            }
        }
    }
}

#[allow(unused)]
#[inline(always)]
fn lock_swap_block_chain(retries: u8) -> Result<LockResult<SwapBlockChainLock>, BusinessError> {
    assert!(retries < 10, "Too many retries");

    match with_mut_state(|s| s.business_swap_block_chain_lock()) {
        Some(lock) => Ok(LockResult::Locked(lock)),
        None => {
            if 0 < retries {
                Ok(LockResult::Retry(retries - 1))
            } else {
                Err(BusinessError::SwapBlockChainLocked)
            }
        }
    }
}

#[allow(unused)]
#[inline(always)]
fn lock_token_block_chain_and_token_balances(
    fee_tokens: Vec<CanisterId>,
    mut required: Vec<TokenAccount>,
    retries: u8,
) -> Result<LockResult<(TokenBlockChainLock, TokenBalancesLock)>, BusinessError> {
    let token_lock = match lock_token_block_chain(retries)? {
        LockResult::Locked(lock) => lock,
        LockResult::Retry(retries) => return Ok(LockResult::Retry(retries)),
    };

    // add fee token account
    if let Some(fee_to) = token_lock.fee_to {
        for token in fee_tokens {
            required.push(TokenAccount {
                token,
                account: fee_to,
            });
        }
    }

    let balances_lock = match lock_token_balances(required, retries)? {
        LockResult::Locked(lock) => lock,
        LockResult::Retry(retries) => {
            drop(token_lock); // ! must drop before retry
            return Ok(LockResult::Retry(retries));
        }
    };

    Ok(LockResult::Locked((token_lock, balances_lock)))
}

#[allow(unused)]
#[inline(always)]
fn lock_token_block_chain_and_swap_block_chain_and_token_balances(
    fee_tokens: Vec<CanisterId>,
    mut required: Vec<TokenAccount>,
    retries: u8,
) -> Result<LockResult<(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock)>, BusinessError>
{
    let token_lock = match lock_token_block_chain(retries)? {
        LockResult::Locked(lock) => lock,
        LockResult::Retry(retries) => return Ok(LockResult::Retry(retries)),
    };

    let swap_lock = match lock_swap_block_chain(retries)? {
        LockResult::Locked(lock) => lock,
        LockResult::Retry(retries) => {
            drop(token_lock); // ! must drop before retry
            return Ok(LockResult::Retry(retries));
        }
    };

    // add fee token account
    if let Some(fee_to) = token_lock.fee_to {
        for &token in &fee_tokens {
            required.push(TokenAccount {
                token,
                account: fee_to,
            });
        }
    }
    // add fee token account
    if let Some(fee_to) = swap_lock.fee_to {
        for &token in &fee_tokens {
            required.push(TokenAccount {
                token,
                account: fee_to,
            });
        }
    }

    let balances_lock = match lock_token_balances(required, retries)? {
        LockResult::Locked(lock) => lock,
        LockResult::Retry(retries) => {
            drop(token_lock); // ! must drop before retry
            drop(swap_lock); // ! must drop before retry
            return Ok(LockResult::Retry(retries));
        }
    };

    Ok(LockResult::Locked((token_lock, swap_lock, balances_lock)))
}

// Query the latest update time
#[ic_cdk::query]
fn updated() -> u64 {
    with_state(|s| s.business_updated())
}
