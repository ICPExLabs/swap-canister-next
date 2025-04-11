use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;

use crate::proto;

impl From<Account> for proto::Account {
    fn from(value: Account) -> Self {
        let owner = value.owner.as_slice().to_vec().into();
        let subaccount = value.subaccount.and_then(|subaccount| {
            if subaccount == [0; 32] {
                None
            } else {
                Some(subaccount.to_vec().into())
            }
        });
        Self { owner, subaccount }
    }
}

impl TryFrom<proto::Account> for Account {
    type Error = String;

    fn try_from(value: proto::Account) -> Result<Self, Self::Error> {
        let owner = Principal::from_slice(&value.owner);
        let subaccount = if let Some(sub) = value.subaccount {
            if sub.len() != 32 {
                return Err("length of subsccount of account must be 32".into());
            }
            let mut subaccount = [0; 32];
            subaccount.copy_from_slice(&sub[..32]);
            Some(subaccount)
        } else {
            None
        };
        Ok(Self { owner, subaccount })
    }
}
