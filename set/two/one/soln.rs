extern crate cryptopals;

use cryptopals::Bytes;
use cryptopals::padding;

fn main() {
  let mut bs = Bytes::from_str("YELLOW SUBMARINE");

  let amt = padding::pkcs7(bs.mut_vec(), 20u8);
  
  println!("padded {} bytes", amt);
  assert!(amt == 4);
  println!("result: {}", bs);
}
