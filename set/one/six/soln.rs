#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin, link)]
extern crate cryptopals;
//#[phase(plugin, link)]
//extern crate static_mdo;

use std::io::File;

use cryptopals::Bytes;
//use cryptopals::byte;
//use cryptopals::analysis::english;

fn ham_test() -> bool {
  let a = Bytes::from_str("this is a test");
  let b = Bytes::from_str("wokka wokka!!!");
  let common = a.n_common_bits(&b)
                .ok()
                .expect("failed to get common bits");

  common == 37  
}

fn main() {
  let mut file = File::open(&Path::new("6.txt"));

  let input = file.read_to_end()
                  .ok()
                  .expect("failed to read input file");

  assert!(ham_test());

  println!("{}", input.get(0));
}
