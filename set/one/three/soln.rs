#![feature(phase)]

extern crate cryptopals;

#[phase(plugin, link)]
extern crate static_mdo;

use cryptopals::Bytes;
use cryptopals::Byte;
use cryptopals::analysis;
use cryptopals::byte;

fn main() {
  let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
  println!("input: {}", input);

  let bs = Bytes::from_hex_str(input).unwrap();
  println!("bs: {}", bs);
  for b in byte::all() {
    let xord = bs.xor_byte(b);
    if xord.is_printable_ascii() {
      println!("bs^'{}' = {}", Byte(b), xord);
    }
  }
}
