#![crate_name = "cryptopals"]
#![crate_type = "rlib"]
#![feature(custom_derive)]
#![feature(collections)]
#![feature(core)]
#![feature(io)]
#![feature(convert)]

#[macro_use]
extern crate bitflags;

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
pub mod block;


#[macro_export]
macro_rules! errln(
  ($($arg:tt)*) => {
    (writeln![std::io::stderr(), $($arg)*]).ok().expect("errln! failed")
  }
);

#[macro_export]
macro_rules! err(
  ($($arg:tt)*) => {
    (write![std::io::stderr(), $($arg)*]).ok().expect("err! failed")
  }
);
