use super::*;

// config
mod config;
#[allow(unused)]
pub use config::*;

// token
mod token;
#[allow(unused)]
pub use token::*;

// pair
mod pair;
#[allow(unused)]
pub use pair::*;

#[allow(unused)]
#[derive(Debug, Deserialize, CandidType)]
pub struct BusinessResult(Result<(), BusinessError>);

impl From<Result<(), BusinessError>> for BusinessResult {
    fn from(value: Result<(), BusinessError>) -> Self {
        Self(value)
    }
}
