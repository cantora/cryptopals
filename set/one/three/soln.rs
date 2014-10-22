extern crate cryptopals;

use cryptopals::Bytes;
use cryptopals::byte;
use cryptopals::analysis::english::find::key;

fn main() {
  let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
  let mut keys: Vec<Bytes> = Vec::new();

  for b in byte::all() {
    keys.push(Bytes::from_byte(b));
  }

  println!("input: {}", input);

  let bs = Bytes::from_hex(&input).unwrap();
  println!("bs: {}", bs);

  let (score, dec, key) = key::best_xor(
    &bs, keys.as_slice()
  ).unwrap();

  println!("{} (key = {}): {}", score, key, dec);
}
