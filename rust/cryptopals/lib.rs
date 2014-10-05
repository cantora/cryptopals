#![crate_id = "cryptopals#0.0.0"]
#![crate_type = "rlib"]
#![feature(macro_rules)]

pub use byte::Byte;
pub use bytes::Bytes;

pub mod byte;
pub mod bytes;

pub mod analysis;
