#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin, link)]
extern crate cryptopals;
//#[phase(plugin, link)]
//extern crate static_mdo;

use std::io::File;
use std::vec;

use cryptopals::byte;
use cryptopals::byte::Byte;
use cryptopals::Bytes;
use cryptopals::combinations::RandomSlice;
use cryptopals::analysis::english::find::key;

fn ham_test() -> bool {
  let a = Bytes::from_str("this is a test");
  let b = Bytes::from_str("wokka wokka!!!");
  let common = a.n_diff_bits(&b)
                .ok()
                .expect("failed to get common bits");

  common == 37  
}

fn avg_ham_for_key_size(sz: uint, dec_vec: &vec::Vec<u8>) -> f64 {
  let mut sum   = 0f64;
  let mut count = 0u;

  for (sl_a, sl_b) in RandomSlice::new(sz, dec_vec).take(15000) {
    let ham = Bytes::from_slice(sl_a).hamming_distance(&Bytes::from_slice(sl_b)).unwrap();
    //println!("{} vs {} = {}", sl_a, sl_b, ham);
    sum += ham;
    count += 1;
  }

  sum/(count as f64)
}

fn main() {
  let mut file = File::open(&Path::new("6.txt"));
  let input = file.read_to_end()
                  .ok()
                  .expect("failed to read input file");
  let mut key_sz     = 2u;
  let mut key_sz_avg = 1.0f64;
  let mut idx = 0u;
  let mut keys: Vec<Bytes> = Vec::new();

  for b in byte::all() {
    keys.push(Bytes::from_byte(b));
  }

  assert!(ham_test());
  let decoded = Bytes::from_base64(&input.as_slice()).unwrap();
  let Bytes(ref dec_vec) = decoded;

  println!("input is {} bytes long", dec_vec.len());
  for sz in range(2u, 40) {
    let avg = avg_ham_for_key_size(sz, dec_vec);
    if avg < key_sz_avg {
      key_sz_avg = avg;
      key_sz     = sz;
    }
    println!("{}: {}", sz, avg);
  }

  println!("best key size ({}): {}", key_sz_avg, key_sz);

  let mut keybytes: Vec<u8> = Vec::new();
  for vec in decoded.transposed(key_sz) {
    let bs = Bytes(vec);
    let (score, _, key) = key::best_xor(
      &bs,
      keys.as_slice()
    ).unwrap();
    let Bytes(ref key_vec) = *key;
    assert!(key_vec.len() == 1);
    let kb = key_vec[0];

    println!("key byte {}: {}", idx, Byte(kb));
    keybytes.push(kb);
    idx += 1;
  }

  let kbs = Bytes(keybytes);
  println!("kbs: {}", kbs);

  println!("decryption:");
  let decrypted = decoded.xor_bytes(&kbs).unwrap();
  println!("{}", decrypted);
}
