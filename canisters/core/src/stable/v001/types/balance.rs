use ic_canister_kit::common::trap;
use icrc_ledger_types::icrc1::account::DEFAULT_SUBACCOUNT;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    sync::RwLock,
};

use super::*;

use super::super::super::with_mut_state_without_record;

// ============================ token account ============================

#[derive(
    Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize, CandidType,
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

// ============================ balance ============================

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct TokenBalance(Nat);

impl Storable for TokenBalance {
    fn to_bytes(&self) -> Cow<[u8]> {
        use ic_canister_kit::common::trap;
        let bytes = candid::encode_one(self.0.clone());
        Cow::Owned(trap(bytes))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        use ic_canister_kit::common::trap;
        let nat = candid::decode_one(&bytes);
        Self(trap(nat))
    }

    const BOUND: Bound = Bound::Unbounded;
}

// ============================ balances ============================

#[derive(Serialize, Deserialize)]
pub struct TokenBalances {
    #[serde(skip, default = "init_token_balances")]
    balances: StableBTreeMap<TokenAccount, TokenBalance>,
    locks: RwLock<HashMap<TokenAccount, bool>>,
}

impl Default for TokenBalances {
    fn default() -> Self {
        Self {
            balances: init_token_balances(),
            locks: Default::default(),
        }
    }
}

impl TokenBalances {
    pub fn token_balance_of(
        &self,
        token: CanisterId,
        account: Account,
    ) -> Result<candid::Nat, BusinessError> {
        let token_account = TokenAccount::new(token, account);
        Ok(self
            .balances
            .get(&token_account)
            .map(|b| b.0)
            .unwrap_or_default())
    }

    // locks
    pub fn lock(
        &mut self,
        fee_to: Vec<TokenAccount>,
        mut required: Vec<TokenAccount>,
    ) -> Result<TokenBalancesLock, Vec<TokenAccount>> {
        let mut locks = trap(self.locks.write()); // ! what if failed ?

        // 0. 手续费账户
        for fee_to in &fee_to {
            required.push(fee_to.clone());
        }

        // duplicate removal
        let locked = required.iter().cloned().collect::<HashSet<_>>();

        // 1. check first
        let mut already_locked: Vec<TokenAccount> = vec![];
        for token_account in &locked {
            if locks.get(token_account).is_some_and(|lock| *lock) {
                already_locked.push(token_account.clone());
            }
        }
        if !already_locked.is_empty() {
            return Err(already_locked);
        }

        // 2. do lock
        for token_account in &locked {
            locks.insert(token_account.clone(), true);
        }

        for account in &required {
            ic_cdk::println!(
                "🔒 Locked token account: [{}]({}.{})",
                account.token.to_text(),
                account.account.owner.to_text(),
                account
                    .account
                    .subaccount
                    .map(hex::encode)
                    .unwrap_or_default()
            );
        }

        Ok(TokenBalancesLock {
            required,
            fee_to,
            locked,
        })
    }

    pub fn unlock(&mut self, locked: &HashSet<TokenAccount>) {
        let mut locks = trap(self.locks.write()); // ! what if failed ?

        // 1. check first
        for token_account in locked {
            if locks.get(token_account).is_some_and(|lock| *lock) {
                continue; // locked is right
            }
            // if not true, terminator
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
            ic_cdk::trap(&tips); // never be here
        }

        // 2. do unlock
        for token_account in locked {
            locks.remove(token_account);
        }
    }

    pub fn be_guard<'a>(&'a mut self, lock: &'a TokenBalancesLock) -> TokenBalancesGuard<'a> {
        TokenBalancesGuard {
            balances: &mut self.balances,
            lock,
        }
    }
}

// ============================ lock ============================

pub struct TokenBalancesLock {
    required: Vec<TokenAccount>,   // 目标要求锁住的账户，打印显示
    fee_to: Vec<TokenAccount>,     // 本次锁是否涉及到的手续费账户
    locked: HashSet<TokenAccount>, // fee_to must be included
}
impl Drop for TokenBalancesLock {
    fn drop(&mut self) {
        with_mut_state_without_record(|s| {
            s.get_mut().business_token_balance_unlock(&self.locked);
            for account in &self.required {
                ic_cdk::println!(
                    "🔐 Unlock token account: [{}]({}.{})",
                    account.token.to_text(),
                    account.account.owner.to_text(),
                    account
                        .account
                        .subaccount
                        .map(hex::encode)
                        .unwrap_or_default()
                );
            }
        })
    }
}

// ============================ guard ============================

pub struct TokenBalancesGuard<'a> {
    balances: &'a mut StableBTreeMap<TokenAccount, TokenBalance>,
    lock: &'a TokenBalancesLock,
}

impl TokenBalancesGuard<'_> {
    #[inline]
    fn do_token_deposit(&mut self, token_account: TokenAccount, amount: Nat) {
        let balance = self.balances.get(&token_account).unwrap_or_default();

        let new_balance = TokenBalance(balance.0 + amount);
        self.balances.insert(token_account, new_balance);
    }
    #[inline]
    fn do_token_withdraw(&mut self, token_account: TokenAccount, amount: Nat) {
        let balance = self.balances.get(&token_account).unwrap_or_default();
        assert!(amount <= balance.0, "Insufficient balance.");

        let new_balance = TokenBalance(balance.0 - amount);
        if new_balance.0 == 0_u64 {
            self.balances.remove(&token_account);
        } else {
            self.balances.insert(token_account, new_balance);
        }
    }

    pub fn token_balance_of(
        &self,
        token: CanisterId,
        account: Account,
    ) -> Result<candid::Nat, BusinessError> {
        let token_account = TokenAccount::new(token, account);
        if !self.lock.locked.contains(&token_account) {
            return Err(BusinessError::Unlocked(vec![token_account]));
        }

        let token_account = TokenAccount::new(token, account);
        Ok(self
            .balances
            .get(&token_account)
            .map(|b| b.0)
            .unwrap_or_default())
    }

    pub fn fee_to(&self) -> &[TokenAccount] {
        &self.lock.fee_to
    }

    // deposit token
    pub fn token_deposit(
        &mut self,
        token: CanisterId,
        account: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        let token_account = TokenAccount::new(token, account);
        if !self.lock.locked.contains(&token_account) {
            return Err(BusinessError::Unlocked(vec![token_account]));
        }
        self.do_token_deposit(token_account, amount); // do deposit
        Ok(())
    }
    // withdraw token
    pub fn token_withdraw(
        &mut self,
        token: CanisterId,
        account: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        let token_account = TokenAccount::new(token, account);
        if !self.lock.locked.contains(&token_account) {
            return Err(BusinessError::Unlocked(vec![token_account]));
        }
        self.do_token_withdraw(token_account, amount); // do withdraw
        Ok(())
    }
    // transfer token
    pub fn token_transfer(
        &mut self,
        token: CanisterId,
        from: Account,
        to: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        let token_account_from = TokenAccount::new(token, from);
        if !self.lock.locked.contains(&token_account_from) {
            return Err(BusinessError::Unlocked(vec![token_account_from]));
        }
        let token_account_to = TokenAccount::new(token, to);
        if !self.lock.locked.contains(&token_account_to) {
            return Err(BusinessError::Unlocked(vec![token_account_to]));
        }

        let from_balance = self.balances.get(&token_account_from).unwrap_or_default();
        assert!(amount <= from_balance.0, "Insufficient balance.");

        self.do_token_withdraw(token_account_from, amount.clone());
        self.do_token_deposit(token_account_to, amount);

        Ok(())
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
