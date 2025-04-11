use super::*;

// token
mod token;
#[allow(unused)]
pub use token::*;

// pair
mod pair;
#[allow(unused)]
pub use pair::*;

#[derive(Debug, Deserialize, CandidType)]
pub struct BusinessResult(Result<(), BusinessError>);

impl From<Result<(), BusinessError>> for BusinessResult {
    fn from(value: Result<(), BusinessError>) -> Self {
        Self(value)
    }
}
