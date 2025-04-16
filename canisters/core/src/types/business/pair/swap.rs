use super::super::*;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapTokensSuccess {
    pub amounts: Vec<Nat>,
}
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapTokensSuccessView {
    pub amounts: Vec<String>,
}
impl From<&TokenPairSwapTokensSuccess> for TokenPairSwapTokensSuccessView {
    fn from(value: &TokenPairSwapTokensSuccess) -> Self {
        Self {
            amounts: value.amounts.iter().map(|a| a.to_string()).collect(),
        }
    }
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct TokenPairSwapTokensResult(Result<TokenPairSwapTokensSuccess, BusinessError>);

impl From<Result<TokenPairSwapTokensSuccess, BusinessError>> for TokenPairSwapTokensResult {
    fn from(value: Result<TokenPairSwapTokensSuccess, BusinessError>) -> Self {
        Self(value)
    }
}

impl From<TokenPairSwapTokensResult> for Result<TokenPairSwapTokensSuccess, BusinessError> {
    fn from(value: TokenPairSwapTokensResult) -> Self {
        value.0
    }
}

/// 检查币对是否首尾相连
/// 上一个币对换回来的结果，正好是下一个币对的输入
pub fn check_path(path: &[SwapTokenPair]) -> Result<(), BusinessError> {
    // check path
    if path.is_empty() {
        return Err(BusinessError::Swap("INVALID_PATH".into()));
    }
    if 1 < path.len() {
        // 循环检查代币是否相连
        let mut i = 1;
        loop {
            if path.len() <= i {
                break;
            }

            let path0 = &path[i - 1];
            let path1 = &path[i];

            if path0.token.1 != path1.token.0 {
                return Err(BusinessError::Swap("INVALID_PATH".into()));
            }

            i += 1;
        }
    }
    Ok(())
}

// ========================= swap by pay exact tokens =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapExactTokensForTokensArgs {
    pub from: Account, // 标记来源，caller 务必和 from 一致

    pub amount_in: Nat,      // pay
    pub amount_out_min: Nat, // min got
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapExactTokensForTokensArg {
    pub self_canister: SelfCanister,
    pub pas: Vec<TokenPairAmm>,

    pub from: Account,
    pub amount_in: Nat,      // pay
    pub amount_out_min: Nat, // min got
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
}

impl SelfCanisterArg for TokenPairSwapExactTokensForTokensArg {
    fn get_self_canister(&self) -> SelfCanister {
        self.self_canister
    }
}

impl TokenPairSwapArg for TokenPairSwapExactTokensForTokensArg {
    fn get_pas(&self) -> &[TokenPairAmm] {
        &self.pas
    }

    fn get_path(&self) -> &[SwapTokenPair] {
        &self.path
    }
}

// ========================= swap by got exact tokens =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapTokensForExactTokensArgs {
    pub from: Account, // 标记来源，caller 务必和 from 一致

    pub amount_out: Nat,    // got
    pub amount_in_max: Nat, // max pay
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapTokensForExactTokensArg {
    pub self_canister: SelfCanister,
    pub pas: Vec<TokenPairAmm>,

    pub from: Account,
    pub amount_out: Nat,    // got
    pub amount_in_max: Nat, // max pay
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
}

impl SelfCanisterArg for TokenPairSwapTokensForExactTokensArg {
    fn get_self_canister(&self) -> SelfCanister {
        self.self_canister
    }
}

impl TokenPairSwapArg for TokenPairSwapTokensForExactTokensArg {
    fn get_pas(&self) -> &[TokenPairAmm] {
        &self.pas
    }

    fn get_path(&self) -> &[SwapTokenPair] {
        &self.path
    }
}
