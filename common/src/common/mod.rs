/// 时间戳
mod timestamp;
pub use timestamp::*;

/// 块的 hash
/// 内容 hash 的 hash 才能成为块的 hash，因为在对外证明时，要求提供后续的所有内容才能证明是不合适的
mod hash;
pub use hash::*;

mod account;
mod nat;
mod principal;

/// block
mod block;
pub use block::*;

/// 记录每笔交易 id
pub type BlockIndex = u64;
