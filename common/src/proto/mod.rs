#[allow(missing_docs)]
pub mod common;

#[allow(missing_docs)]
pub mod block;

#[cfg(feature = "archive-token")]
#[allow(missing_docs)]
pub mod token;
#[cfg(feature = "archive-token")]
pub use token::*;

#[cfg(feature = "archive-swap")]
#[allow(missing_docs)]
#[allow(clippy::large_enum_variant)]
pub mod swap;
#[cfg(feature = "archive-swap")]
pub use swap::*;

pub use block::*;
pub use common::*;
pub use prost::Message;
pub use prost::bytes::Bytes;

impl From<Vec<u8>> for EncodedBlock {
    fn from(value: Vec<u8>) -> Self {
        Self { block: value.into() }
    }
}

#[allow(missing_docs)]
#[cfg(test)]
pub mod test;
