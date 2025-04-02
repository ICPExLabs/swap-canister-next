use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

use candid::{Nat, Principal};
use ic_canister_kit::types::{Bound, CanisterId, StableBTreeMap, Storable};
use icrc_ledger_types::icrc1::account::{Account, DEFAULT_SUBACCOUNT};

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct TokenAccount {
    pub canister_id: CanisterId,
    pub account: Account,
}

impl TokenAccount {
    pub fn new(canister_id: CanisterId, account: Account) -> Self {
        Self {
            canister_id,
            account,
        }
    }
}

impl Storable for TokenAccount {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut bytes = vec![];

        // push canister_id // 1 + Principal::MAX_LENGTH_IN_BYTES
        let canister_id_bytes = self.canister_id.as_slice();
        bytes.push(canister_id_bytes.len() as u8);
        bytes.extend_from_slice(canister_id_bytes);

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
        let canister_id_len = bytes[0] as usize;
        let canister_id = CanisterId::from_slice(&bytes[1..1 + canister_id_len]);

        // read owner
        let offset = 1 + canister_id_len;
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
            canister_id,
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

pub type TokenBalances = StableBTreeMap<TokenAccount, TokenBalance>;
pub type TokenBalanceLocks = HashMap<TokenAccount, bool>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_account() {
        let canister_id = CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let owner = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let account = Account {
            owner,
            subaccount: Some([1; 32]),
        };
        let token_account = TokenAccount {
            canister_id,
            account,
        };

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
