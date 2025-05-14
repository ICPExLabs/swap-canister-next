use common::types::{BusinessError, TokenPair};

use crate::types::{Business, with_state};

pub fn check_token_pair_args(_self: &TokenPair) -> Result<(), BusinessError> {
    // check supported token
    with_state(|s| {
        // ! must be token, can not be dummy lp token
        let tokens = s.business_tokens_query();
        if !tokens.contains_key(&_self.get_token0()) {
            return Err(BusinessError::NotSupportedToken(_self.get_token0()));
        }
        if !tokens.contains_key(&_self.get_token1()) {
            return Err(BusinessError::NotSupportedToken(_self.get_token1()));
        }

        // must be different
        if _self.get_token0() == _self.get_token1() {
            return Err(BusinessError::InvalidTokenPair(_self.get_token0(), _self.get_token1()));
        }

        Ok(())
    })
}
