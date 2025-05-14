use crate::{
    proto,
    types::{TokenPair, TokenPairAmm},
};

// ======================== token pair ========================

impl From<TokenPair> for proto::TokenPair {
    fn from(value: TokenPair) -> Self {
        let token0 = value.get_token0().into();
        let token1 = value.get_token1().into();
        Self {
            token0: Some(token0),
            token1: Some(token1),
        }
    }
}

impl TryFrom<proto::TokenPair> for TokenPair {
    type Error = String;

    fn try_from(value: proto::TokenPair) -> Result<Self, Self::Error> {
        let token0 = value
            .token0
            .ok_or_else(|| "token0 of token pair can not be none".to_string())?
            .into();
        let token1 = value
            .token1
            .ok_or_else(|| "token1 of token pair can not be none".to_string())?
            .into();
        Ok(Self::new(token0, token1))
    }
}

// ======================== token pair amm ========================

impl From<TokenPairAmm> for proto::TokenPairAmm {
    fn from(value: TokenPairAmm) -> Self {
        let pair = value.pair.into();
        let amm = value.amm.into();
        Self { pair: Some(pair), amm }
    }
}

impl TryFrom<proto::TokenPairAmm> for TokenPairAmm {
    type Error = String;

    fn try_from(value: proto::TokenPairAmm) -> Result<Self, Self::Error> {
        let pair = value
            .pair
            .ok_or_else(|| "pair of token pair amm can not be none".to_string())?
            .try_into()?;
        let amm = value.amm.as_str().try_into().map_err(|err| format!("{err:?}"))?;
        Ok(Self { pair, amm })
    }
}
