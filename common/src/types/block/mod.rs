/// hash of block
/// The hash of the content hash can become a hash of the block, because when proofing externally, it is inappropriate to require all subsequent content to prove it
mod hash;
pub use hash::*;

/// block
#[allow(clippy::module_inception)]
mod block;
pub use block::*;

/// query
mod query;
pub use query::*;

/// Record each transaction id
pub type BlockIndex = u64;

/// Maximum number of requested blocks
pub const MAX_BLOCKS_PER_REQUEST: u64 = 2_000;
