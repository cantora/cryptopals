extern crate cryptopals;

use cryptopals::Bytes;
use cryptopals::byte;
use cryptopals::analysis::english;

fn main() {
  let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

  println!("input: {}", input);

  let bs = Bytes::from_hex(&input).unwrap();
  println!("bs: {}", bs);

  let mut engl_heap = english::HeapByChi2::new();

  for b in byte::all() {
    engl_heap.add(bs.xor_byte(b), b);
  }

  for (score, val, key) in engl_heap.consume().take(5) {
    println!("{} (key = {}): {}", score, key, val);
  }
}
