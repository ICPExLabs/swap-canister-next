use icrc_ledger_types::icrc1::account::DEFAULT_SUBACCOUNT;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use super::*;

use super::super::super::with_mut_state_without_record;

#[derive(
    Debug, Serialize, Deserialize, CandidType, Hash, Eq, PartialEq, Clone, PartialOrd, Ord,
)]
pub struct TokenAccount {
    pub token: CanisterId,
    pub account: Account,
}

impl TokenAccount {
    pub fn new(token: CanisterId, account: Account) -> Self {
        Self { token, account }
    }
}

impl Storable for TokenAccount {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut bytes = vec![];

        // push canister_id // 1 + Principal::MAX_LENGTH_IN_BYTES
        let token_bytes = self.token.as_slice();
        bytes.push(token_bytes.len() as u8);
        bytes.extend_from_slice(token_bytes);

        // push owner // Principal::MAX_LENGTH_IN_BYTES
        let owner_bytes = self.account.owner.as_slice();
        bytes.extend_from_slice(owner_bytes);

        // push subaccount // ? 32
        let subaccount = self
            .account
            .subaccount
            .as_ref()
            .unwrap_or(DEFAULT_SUBACCOUNT);
        if DEFAULT_SUBACCOUNT != subaccount {
            bytes.extend_from_slice(subaccount);
        }

        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        // read canister_id
        let token_len = bytes[0] as usize;
        let token = CanisterId::from_slice(&bytes[1..1 + token_len]);

        // read owner
        let offset = 1 + token_len;
        let remain = bytes.len() - offset;
        let owner_len = if 32 < remain { remain - 32 } else { remain };
        let owner = Principal::from_slice(&bytes[offset..offset + owner_len]);

        // read subaccount
        let offset = offset + owner_len;
        let subaccount = if offset < bytes.len() {
            let mut subaccount = [0; 32];
            subaccount.copy_from_slice(&bytes[offset..offset + 32]);
            Some(subaccount)
        } else {
            None
        };

        Self {
            token,
            account: Account { owner, subaccount },
        }
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1
            + Principal::MAX_LENGTH_IN_BYTES as u32 // canister_id
            + Principal::MAX_LENGTH_IN_BYTES as u32 // owner
            + 32, // subaccount
        is_fixed_size: false,
    };
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct TokenBalance(Nat);

impl Storable for TokenBalance {
    fn to_bytes(&self) -> Cow<[u8]> {
        #[allow(clippy::unwrap_used)] // ? SAFETY
        let bytes = candid::encode_one(self.0.clone()).unwrap();
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        #[allow(clippy::unwrap_used)] // ? SAFETY
        let nat = candid::decode_one(&bytes).unwrap();
        Self(nat)
    }

    const BOUND: Bound = Bound::Unbounded;
}

pub struct TokenBalances(StableBTreeMap<TokenAccount, TokenBalance>);
#[derive(Serialize, Deserialize, Default)]
pub struct TokenBalanceLocks(HashMap<TokenAccount, bool>);

pub struct TokenBalanceLockGuard<'a>(&'a [TokenAccount]);
impl Drop for TokenBalanceLockGuard<'_> {
    fn drop(&mut self) {
        with_mut_state_without_record(|s| s.get_mut().business_token_balance_unlock(self.0))
    }
}

impl TokenBalances {
    pub fn new(inner: StableBTreeMap<TokenAccount, TokenBalance>) -> Self {
        Self(inner)
    }

    pub fn token_balance_of(&self, token: CanisterId, account: Account) -> candid::Nat {
        let token_account = TokenAccount::new(token, account);
        self.0.get(&token_account).map(|b| b.0).unwrap_or_default()
    }

    // token deposit and withdraw
    pub fn token_deposit(&mut self, token: CanisterId, account: Account, amount: Nat) {
        let token_account = TokenAccount::new(token, account);
        let balance = self.0.get(&token_account).unwrap_or_default();
        let new_balance = TokenBalance(balance.0 + amount);
        self.0.insert(token_account, new_balance);
    }
    pub fn token_withdraw(&mut self, token: CanisterId, account: Account, amount: Nat) {
        let token_account = TokenAccount::new(token, account);
        let balance = self.0.get(&token_account).unwrap_or_default();
        #[allow(clippy::panic)] // ? SAFETY
        if balance.0 < amount {
            panic!("Insufficient balance.");
        }
        let new_balance = TokenBalance(balance.0 - amount);
        if new_balance.0 == 0_u64 {
            self.0.remove(&token_account);
        } else {
            self.0.insert(token_account, new_balance);
        }
    }

    // transfer
    pub fn token_transfer(&mut self, token: CanisterId, from: Account, to: Account, amount: Nat) {
        self.token_withdraw(token, from, amount.clone());
        self.token_deposit(token, to, amount);
    }
}

impl TokenBalanceLocks {
    pub fn lock<'a>(
        &mut self,
        token_accounts: &'a [TokenAccount],
    ) -> Result<TokenBalanceLockGuard<'a>, Vec<TokenAccount>> {
        // 0. repeat accounts
        let _token_accounts = token_accounts.iter().collect::<HashSet<_>>();

        // 1. check first
        let mut locked: Vec<TokenAccount> = vec![];
        for &token_account in &_token_accounts {
            if self.0.get(token_account).is_some_and(|lock| *lock) {
                locked.push(token_account.clone());
            }
        }
        if !locked.is_empty() {
            return Err(locked);
        }

        // 2. do lock
        for token_account in _token_accounts {
            self.0.insert(token_account.clone(), true);
        }

        Ok(TokenBalanceLockGuard(token_accounts))
    }

    pub fn unlock(&mut self, token_accounts: &[TokenAccount]) {
        // 0. repeat accounts
        let _token_accounts = token_accounts.iter().collect::<HashSet<_>>();

        // 1. check first
        for &token_account in &_token_accounts {
            if self.0.get(token_account).is_some_and(|lock| *lock) {
                continue; // locked is right
            }
            // if not true, panic
            #[allow(clippy::panic)] // ? SAFETY
            {
                let tips = format!(
                    "Unlock a token account (\"{}|{}.{}\") that is not locked.",
                    token_account.token.to_text(),
                    token_account.account.owner.to_text(),
                    token_account
                        .account
                        .subaccount
                        .map(hex::encode)
                        .unwrap_or_default()
                );
                panic!("{}", tips)
            }
        }

        // then unlock
        for token_account in _token_accounts {
            self.0.remove(token_account);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_account() {
        let token = CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let owner = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let account = Account {
            owner,
            subaccount: Some([1; 32]),
        };
        let token_account = TokenAccount { token, account };

        let bytes = token_account.to_bytes();
        let token_account2 = TokenAccount::from_bytes(bytes);

        assert_eq!(token_account, token_account2);
    }

    #[test]
    fn test_token_balance() {
        let balance = TokenBalance(Nat::from(100_u64));

        let bytes = balance.to_bytes();
        let balance2 = TokenBalance::from_bytes(bytes);

        assert_eq!(balance, balance2);
    }
}
