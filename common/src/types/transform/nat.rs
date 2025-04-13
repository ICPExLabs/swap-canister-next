use candid::Nat;

use crate::proto;

impl TryFrom<Nat> for proto::Nat {
    type Error = candid::Error;

    fn try_from(value: Nat) -> Result<Self, Self::Error> {
        let mut bytes = vec![];
        value.encode(&mut bytes)?;
        Ok(Self {
            bytes: bytes.into(),
        })
    }
}

impl TryFrom<proto::Nat> for Nat {
    type Error = candid::Error;

    fn try_from(value: proto::Nat) -> Result<Self, Self::Error> {
        let mut bytes = std::io::Cursor::new(value.bytes);
        Nat::decode(&mut bytes)
    }
}
