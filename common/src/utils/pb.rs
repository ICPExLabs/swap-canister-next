pub use crate::proto::Message;

/// decoding
pub fn from_proto_bytes<T: prost::Message + Default>(bytes: &[u8]) -> Result<T, String> {
    prost::Message::decode(bytes).map_err(|e| e.to_string())
}

/// encoding
pub fn to_proto_bytes<T: prost::Message + Default>(data: &T) -> Result<Vec<u8>, String> {
    let mut bytes = vec![];
    data.encode(&mut bytes).map_err(|e| e.to_string())?;
    Ok(bytes)
}
