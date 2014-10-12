#![crate_id = "cryptopals#0.0.0"]
#![crate_type = "rlib"]
#![feature(macro_rules)]
#![feature(globs)]

pub use byte::Byte;
pub use bytes::Bytes;
pub use pqueue::PriorityQueue;

mod pqueue;
pub mod byte;
pub mod bytes;
pub mod util;
pub mod analysis;

#[macro_export]
macro_rules! errln(
  ($fmt:expr$(, $msg:expr)*) => {
    (writeln![std::io::stderr(), $fmt $(, $msg)*]).ok().expect("errln! failed")
  }
)

#[macro_export]
macro_rules! err(
  ($fmt:expr$(, $msg:expr)*) => {
    (write![std::io::stderr(), $fmt $(, $msg)*]).ok().expect("errln! failed")
  }
)
