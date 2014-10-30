#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin, link)]
extern crate cryptopals;
extern crate openssl;

use std::io::File;
//use std::vec::Vec;

use openssl::crypto::symm;

//use cryptopals::byte;
//use cryptopals::byte::Byte;
use cryptopals::Bytes;
//use cryptopals::combinations::RandomSlice;
//use cryptopals::analysis::english::find::key;



fn main() {
  let mut file = File::open(&Path::new("7.txt"));
  let input = file.read_to_end()
                  .ok()
                  .expect("failed to read input file");

  let decoded    = Bytes::from_base64(&input.as_slice()).unwrap();
  let crypter    = symm::Crypter::new(symm::AES_128_ECB);
  let key        = "YELLOW SUBMARINE".as_bytes();
  let block_sz   = 16;
  let pt_len     = decoded.len();

  println!("input is {} bytes long", pt_len);
  if pt_len % block_sz != 0 {
    fail!("input is not a multible of 16 bytes!");
  }

  let n_blocks = pt_len/block_sz;
  if n_blocks < 1 {
    fail!("input is too small");
  }

  crypter.init(symm::Decrypt, key, vec![]);
  crypter.pad(false);
  for block in decoded.as_slice().chunks(n_blocks) {
    let pt = crypter.update(block);
	print!("{}", Bytes(pt));
  }

  let remainder = Bytes(crypter.finalize());
  println!("{}", remainder);
}
