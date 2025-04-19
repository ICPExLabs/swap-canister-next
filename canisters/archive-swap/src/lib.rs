#![doc = include_str!("../README.md")]
// #![deny(unreachable_pub)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(clippy::future_not_send)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

mod types;

mod stable;

mod business;

mod http;

mod common; // must at last cause candid
