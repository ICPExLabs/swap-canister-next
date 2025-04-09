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
pub enum BusinessResult {
    Ok(()),
    Err(BusinessError),
}
impl From<Result<(), BusinessError>> for BusinessResult {
    fn from(r: Result<(), BusinessError>) -> Self {
        match r {
            Ok(n) => BusinessResult::Ok(n),
            Err(e) => BusinessResult::Err(e),
        }
    }
}
