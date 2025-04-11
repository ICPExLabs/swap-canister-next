// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    #[prost(bytes = "bytes", tag = "1")]
    pub hash: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(bytes = "bytes", tag = "1")]
    pub owner: ::prost::bytes::Bytes,
    #[prost(bytes = "bytes", optional, tag = "2")]
    pub subaccount: ::core::option::Option<::prost::bytes::Bytes>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanisterId {
    #[prost(bytes = "bytes", tag = "1")]
    pub bytes: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nat {
    #[prost(bytes = "bytes", tag = "1")]
    pub bytes: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserId {
    #[prost(bytes = "bytes", tag = "1")]
    pub bytes: ::prost::bytes::Bytes,
}
