#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin, link)]
extern crate cryptopals;

use std::io::File;

use cryptopals::Bytes;
use cryptopals::crypto::sym::mode::cbc;

fn main() {
  let mut file = File::open(&Path::new("10.txt"));
  let input = file.read_to_end()
                  .ok()
                  .expect("failed to read input file");

  let decoded    = Bytes::from_base64(&input.as_slice()).unwrap();
  let key        = "YELLOW SUBMARINE".as_bytes();
  let iv         = [0, ..16];
  let mut cbc    = cbc::DecStream::aes_128_dec(key, iv.to_vec());
  let pt_len     = decoded.len();

  println!("input is {} bytes long", pt_len);

  for pt in cbc.iter(&mut decoded.as_slice().iter()) {
    print!("{}", Bytes(pt));
  }
  match cbc.finish() {
    Ok(tail) => print!("{}", Bytes(tail)),
    Err(buf) => print!("\npadding error: {}", Bytes(buf))
  }
  print!("\n");
}
