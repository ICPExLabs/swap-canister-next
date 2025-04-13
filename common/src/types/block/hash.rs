use candid::types::internal::{Type, TypeInner};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use candid::CandidType;

use crate::proto;

/// The length of a block/transaction hash in bytes.
pub const HASH_LENGTH: usize = 32;

/// hash
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashOf<T> {
    inner: [u8; HASH_LENGTH],
    _marker: PhantomData<T>,
}

impl<T> Serialize for HashOf<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}
impl<'de, T> Deserialize<'de> for HashOf<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = <[u8; HASH_LENGTH]>::deserialize(deserializer)?;
        Ok(Self {
            inner,
            _marker: PhantomData,
        })
    }
}

impl<T> CandidType for HashOf<T> {
    fn _ty() -> Type {
        TypeInner::Vec(TypeInner::Nat8.into()).into()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        serializer.serialize_blob(self.as_slice())
    }
}

impl<T: std::clone::Clone> Copy for HashOf<T> {}

impl<T> Default for HashOf<T> {
    fn default() -> Self {
        Self::new([0; 32])
    }
}

impl<T> HashOf<T> {
    /// Creates a new hash from a byte array.
    pub fn new(bytes: [u8; HASH_LENGTH]) -> Self {
        Self {
            inner: bytes,
            _marker: PhantomData,
        }
    }

    /// 拆开
    pub fn into_bytes(self) -> [u8; HASH_LENGTH] {
        self.inner
    }

    /// 使用
    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    /// hex
    pub fn hex(&self) -> String {
        hex::encode(self.as_slice())
    }
}

impl<T> From<HashOf<T>> for proto::Hash {
    fn from(value: HashOf<T>) -> Self {
        Self {
            hash: value.inner.to_vec().into(),
        }
    }
}

impl<T> TryFrom<proto::Hash> for HashOf<T> {
    type Error = String;

    fn try_from(value: proto::Hash) -> Result<Self, Self::Error> {
        if value.hash.len() != HASH_LENGTH {
            return Err(format!("hash bytes must be: {HASH_LENGTH}"));
        }
        let mut inner = [0; HASH_LENGTH];
        inner.copy_from_slice(&value.hash[..HASH_LENGTH]);
        Ok(Self {
            inner,
            _marker: PhantomData,
        })
    }
}

/// sha256 hash
pub trait DoHash: Sized {
    /// sha256 hash
    fn do_hash(&self) -> Result<HashOf<Self>, String>;
}
