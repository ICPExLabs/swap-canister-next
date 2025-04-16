use ::common::archive::token::TransferFee;
use ic_canister_kit::common::trap;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    sync::RwLock,
};

use super::*;

use super::super::super::with_mut_state_without_record;

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
                ic_cdk::println!("🔐 Unlock token account: {}", account);
            }
        })
    }
}

impl TokenBalancesLock {
    pub fn fee_to(&self) -> &[TokenAccount] {
        &self.fee_to
    }
}

// ============================ guard ============================

pub struct TokenBalancesGuard<'a> {
    balances: &'a mut StableBTreeMap<TokenAccount, TokenBalance>,
    lock: &'a TokenBalancesLock,
}

impl TokenBalancesGuard<'_> {
    pub fn get_locked_balances(&self) -> Vec<TokenAccount> {
        self.lock.required.clone()
    }

    #[inline]
    fn inner_do_token_deposit(&mut self, token_account: TokenAccount, amount: Nat) {
        let balance = self.balances.get(&token_account).unwrap_or_default();

        let new_balance = TokenBalance(balance.0 + amount);
        self.balances.insert(token_account, new_balance);
    }
    #[inline]
    fn inner_token_withdraw(&mut self, token_account: TokenAccount, amount: Nat) {
        let balance = self.balances.get(&token_account).unwrap_or_default();
        assert!(amount <= balance.0, "Insufficient balance.");

        let new_balance = TokenBalance(balance.0 - amount);
        if new_balance.0 == 0_u64 {
            self.balances.remove(&token_account);
        } else {
            self.balances.insert(token_account, new_balance);
        }
    }
    // deposit token
    fn do_token_deposit(
        &mut self,
        token: CanisterId,
        account: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        let token_account = TokenAccount::new(token, account);
        if !self.lock.locked.contains(&token_account) {
            return Err(BusinessError::TokenAccountsUnlocked(vec![token_account]));
        }
        self.inner_do_token_deposit(token_account, amount); // do deposit
        Ok(())
    }
    // withdraw token
    fn do_token_withdraw(
        &mut self,
        token: CanisterId,
        account: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        let token_account = TokenAccount::new(token, account);
        if !self.lock.locked.contains(&token_account) {
            return Err(BusinessError::TokenAccountsUnlocked(vec![token_account]));
        }
        self.inner_token_withdraw(token_account, amount); // do withdraw
        Ok(())
    }
    // transfer token
    fn do_token_transfer(
        &mut self,
        token: CanisterId,
        from: Account,
        amount_without_fee: Nat,
        to: Account,
        fee: Option<TransferFee>,
    ) -> Result<Nat, BusinessError> {
        // check
        let changed = amount_without_fee.clone()
            + fee
                .as_ref()
                .map(|TransferFee { fee, .. }| fee.clone())
                .unwrap_or_default();
        let from_balance = self.token_balance_of(token, from)?;
        if from_balance < changed {
            return Err(BusinessError::insufficient_balance(token, from_balance));
        }

        // do transfer
        self.do_token_withdraw(token, from, amount_without_fee.clone())?; // withdraw
        self.do_token_deposit(token, to, amount_without_fee.clone())?; // deposit

        // fee
        if let Some(TransferFee { fee, fee_to }) = fee {
            // do transfer fee
            self.do_token_withdraw(token, from, fee.clone())?; // withdraw
            self.do_token_deposit(token, fee_to, fee.clone())?; // deposit
        }

        Ok(changed)
    }

    pub fn token_balance_of(
        &self,
        token: CanisterId,
        account: Account,
    ) -> Result<candid::Nat, BusinessError> {
        let token_account = TokenAccount::new(token, account);
        if !self.lock.locked.contains(&token_account) {
            return Err(BusinessError::TokenAccountsUnlocked(vec![token_account]));
        }

        let token_account = TokenAccount::new(token, account);
        Ok(self
            .balances
            .get(&token_account)
            .map(|b| b.0)
            .unwrap_or_default())
    }

    // deposit token
    pub fn token_deposit(
        &mut self,
        guard: &mut TokenBlockChainGuard,
        arg: ArgWithMeta<DepositToken>,
    ) -> Result<(), BusinessError> {
        // 1. get token block
        let transaction = TokenTransaction {
            operation: TokenOperation::Deposit(arg.arg.clone()),
            memo: arg.memo,
            created: arg.created,
        };
        // 2. do deposit and mint block
        guard.mint_block(arg.now, transaction, || {
            self.do_token_deposit(arg.arg.token, arg.arg.to, arg.arg.amount)
        })?;
        Ok(())
    }
    // withdraw token
    pub fn token_withdraw(
        &mut self,
        guard: &mut TokenBlockChainGuard,
        arg: ArgWithMeta<WithdrawToken>,
    ) -> Result<(), BusinessError> {
        // 1. get token block
        let transaction = TokenTransaction {
            operation: TokenOperation::Withdraw(arg.arg.clone()),
            memo: arg.memo,
            created: arg.created,
        };
        // 2. do withdraw and mint block
        guard.mint_block(arg.now, transaction, || {
            self.do_token_withdraw(arg.arg.token, arg.arg.from, arg.arg.amount)
        })?;
        Ok(())
    }
    // transfer token
    pub fn token_transfer(
        &mut self,
        guard: &mut TokenBlockChainGuard,
        arg: ArgWithMeta<TransferToken>,
    ) -> Result<Nat, BusinessError> {
        // 1. get token block
        let transaction = TokenTransaction {
            operation: TokenOperation::Transfer(arg.arg.clone()),
            memo: arg.memo,
            created: arg.created,
        };
        // 2. do transfer and mint block
        let changed = guard.mint_block(arg.now, transaction, || {
            self.do_token_transfer(
                arg.arg.token,
                arg.arg.from,
                arg.arg.amount,
                arg.arg.to,
                arg.arg.fee,
            )
        })?;
        Ok(changed)
    }
    // // transfer token
    // pub fn token_transfer(
    //     &mut self,
    //     token: CanisterId,
    //     from: Account,
    //     to: Account,
    //     amount: Nat,
    // ) -> Result<(), BusinessError> {
    //     let token_account_from = TokenAccount::new(token, from);
    //     if !self.lock.locked.contains(&token_account_from) {
    //         return Err(BusinessError::Unlocked(vec![token_account_from]));
    //     }
    //     let token_account_to = TokenAccount::new(token, to);
    //     if !self.lock.locked.contains(&token_account_to) {
    //         return Err(BusinessError::Unlocked(vec![token_account_to]));
    //     }

    //     let from_balance = self.balances.get(&token_account_from).unwrap_or_default();
    //     assert!(amount <= from_balance.0, "Insufficient balance.");

    //     self.inner_token_withdraw(token_account_from, amount.clone());
    //     self.inner_do_token_deposit(token_account_to, amount);

    //     Ok(())
    // }
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
