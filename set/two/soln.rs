#![feature(phase)]

extern crate cryptopals;

#[phase(plugin, link)]
extern crate static_mdo;

use cryptopals::Bytes;

fn main() {
  let input1 = "1c0111001f010100061a024b53535009181c";
  let input2 = "686974207468652062756c6c277320657965";
  println!("hex1: {}", input1);
  println!("hex2: {}", input2);

  let result = result_do! {
    bs1 <- Bytes::from_hex_str(input1);
    ign println!("bs1: {}", bs1);
    bs2 <- Bytes::from_hex_str(input2);
    ign println!("bs2: {}", bs2);
    ret bs1 ^ bs2
  };

  match result {
    Ok(xor_bs) => println!("result: {}", xor_bs),
    Err(err)   => println!("error {}", err)
  }
}
