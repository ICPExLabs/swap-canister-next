use common::types::{BusinessError, TokenPair};

use crate::types::{Business, with_state};

pub fn check_token_pair_args(_self: &TokenPair) -> Result<(), BusinessError> {
    // check supported token
    with_state(|s| {
        // ! must be token, can not be dummy lp token
        let tokens = s.business_tokens_query();
        if !tokens.contains_key(&_self.token0) {
            return Err(BusinessError::NotSupportedToken(_self.token0));
        }
        if !tokens.contains_key(&_self.token1) {
            return Err(BusinessError::NotSupportedToken(_self.token1));
        }

        // must be different
        if _self.token0 == _self.token1 {
            return Err(BusinessError::InvalidTokenPair(_self.token0, _self.token1));
        }

        Ok(())
    })
}
