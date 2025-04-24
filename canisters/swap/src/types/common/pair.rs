use common::types::{BusinessError, TokenPair};

use crate::types::{Business, CheckArgs, with_state};

impl CheckArgs for TokenPair {
    type Result = ();

    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check supported token
        with_state(|s| {
            // ! must be token, can not be dummy lp token
            let tokens = s.business_tokens_query();
            if !tokens.contains_key(&self.token0) {
                return Err(BusinessError::NotSupportedToken(self.token0));
            }
            if !tokens.contains_key(&self.token1) {
                return Err(BusinessError::NotSupportedToken(self.token1));
            }

            // must be different
            if self.token0 == self.token1 {
                return Err(BusinessError::InvalidTokenPair(self.token0, self.token1));
            }

            Ok(())
        })
    }
}
