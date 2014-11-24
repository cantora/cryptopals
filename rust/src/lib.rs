#![crate_name = "cryptopals"]
#![crate_type = "rlib"]
#![feature(macro_rules)]
#![feature(globs)]

pub use byte::Byte;
pub use bytes::Bytes;
pub use binary_heap::BinaryHeap;

mod binary_heap;
pub mod byte;
pub mod bytes;
pub mod util;
pub mod analysis;
pub mod combinations;
pub mod iter;
pub mod entropy;
pub mod padding;
pub mod crypto;

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
