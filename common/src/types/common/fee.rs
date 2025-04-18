use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::proto;

// ============================== transfer fee ==============================

/// 转账手续费
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct TransferFee {
    /// 手续费数量
    pub fee: Nat,
    /// 手续费收取账户
    pub fee_to: Account,
}

impl TryFrom<TransferFee> for proto::TransferFee {
    type Error = candid::Error;

    fn try_from(value: TransferFee) -> Result<Self, Self::Error> {
        let fee = value.fee.try_into()?;
        let fee_to = value.fee_to.into();
        Ok(Self {
            fee: Some(fee),
            fee_to: Some(fee_to),
        })
    }
}

impl TryFrom<proto::TransferFee> for TransferFee {
    type Error = String;

    fn try_from(value: proto::TransferFee) -> Result<Self, Self::Error> {
        let fee = value
            .fee
            .ok_or_else(|| "from of transfer_fee can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore fee of transfer_fee failed".to_string())?;
        let fee_to = value
            .fee_to
            .ok_or_else(|| "fee_to of transfer_fee can not be none".to_string())?
            .try_into()?;

        Ok(Self { fee, fee_to })
    }
}

// ============================== burn fee ==============================

/// 销毁手续费，流动性销毁需要收取手续费，防止女巫攻击
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct BurnFee {
    /// 手续费数量
    pub fee: Nat,
    /// 手续费收取账户
    pub fee_to: Account,
}

impl TryFrom<BurnFee> for proto::BurnFee {
    type Error = candid::Error;

    fn try_from(value: BurnFee) -> Result<Self, Self::Error> {
        let fee = value.fee.try_into()?;
        let fee_to = value.fee_to.into();
        Ok(Self {
            fee: Some(fee),
            fee_to: Some(fee_to),
        })
    }
}

impl TryFrom<proto::BurnFee> for BurnFee {
    type Error = String;

    fn try_from(value: proto::BurnFee) -> Result<Self, Self::Error> {
        let fee = value
            .fee
            .ok_or_else(|| "from of burn_fee can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore fee of burn_fee failed".to_string())?;
        let fee_to = value
            .fee_to
            .ok_or_else(|| "fee_to of burn_fee can not be none".to_string())?
            .try_into()?;

        Ok(Self { fee, fee_to })
    }
}
