#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin, link)]
extern crate cryptopals;

use std::io::File;

use cryptopals::Bytes;
use cryptopals::crypto::sym::mode::cbc;
use cryptopals::crypto::sym::mode::cbc::Stream;

fn main() {
  let mut file = File::open(&Path::new("10.txt"));
  let input = file.read_to_end()
                  .ok()
                  .expect("failed to read input file");

  let decoded    = Bytes::from_base64(&input.as_slice()).unwrap();
  let key        = "YELLOW SUBMARINE".as_bytes();
  let iv         = [0, ..16];
  let mut d_cbc  = cbc::DecStream::aes_128_dec(key, iv.to_vec());
  let pt_len     = decoded.len();
  let mut e_cbc  = cbc::EncStream::aes_128_enc(key, iv.to_vec());

  let mut encrypted: Vec<u8> = Vec::new();

  println!("input is {} bytes long", pt_len);

  for pt in d_cbc.iter(&mut decoded.as_slice().iter()) {
    for ct in e_cbc.iter(&mut pt.as_slice().iter()) {
      encrypted.push_all(ct.as_slice());
    }
    print!("{}", Bytes(pt));
  }
  match d_cbc.finish() {
    Ok(tail) => {
      for ct in e_cbc.iter(&mut tail.as_slice().iter()) {
        encrypted.push_all(ct.as_slice());
      }
      print!("{}", Bytes(tail))
    }
    Err(buf) => print!("\npadding error: {}", Bytes(buf))
  }
  print!("\n");

  encrypted.push_all(e_cbc.finish().as_slice());
  println!("encrypted == decoded: {}", encrypted == *decoded.vec());
}
