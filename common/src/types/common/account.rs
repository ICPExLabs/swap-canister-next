#[allow(unused)]
use std::{borrow::Cow, fmt::Display};

#[allow(unused)]
use candid::{CandidType, Principal};
#[cfg(feature = "cdk")]
use ic_canister_kit::types::{Bound, Storable};
#[allow(unused)]
use icrc_ledger_types::icrc1::account::{Account, DEFAULT_SUBACCOUNT};
use serde::{Deserialize, Serialize};

use crate::types::CanisterId;

// ============================ token account ============================

/// Token Account
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize, CandidType)]
pub struct TokenAccount {
    /// Token canister
    pub token: CanisterId,
    /// Account
    pub account: Account,
}

/// show Account
pub fn display_account(account: &Account) -> String {
    format!(
        "{}.{}",
        account.owner.to_text(),
        account.subaccount.map(hex::encode).unwrap_or_default()
    )
}

impl Display for TokenAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]({})", self.token.to_text(), display_account(&self.account))
    }
}

impl TokenAccount {
    /// new
    pub fn new(token: CanisterId, account: Account) -> Self {
        Self { token, account }
    }
}

#[cfg(feature = "cdk")]
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
        let subaccount = self.account.subaccount.as_ref().unwrap_or(DEFAULT_SUBACCOUNT);
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
