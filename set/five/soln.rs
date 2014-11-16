#![feature(macro_rules)]
#![feature(phase)]

#[phase(plugin, link)]
extern crate cryptopals;

use cryptopals::bytes::Bytes;

fn main() {
  let input = "Burning 'em, if you ain't quick and nimble\n\
               I go crazy when I hear a cymbal";
  let expected = "0b3637272a2b2e63622c2e69692a2369\
                  3a2a3c6324202d623d63343c2a262263\
                  24272765272a282b2f20430a652e2c65\
                  2a3124333a653e2b2027630c692b2028\
                  3165286326302e27282f";
  let pt = Bytes::from_str(input);

  println!("pt: {}", pt);
  let ct = pt.xor_bytes(&Bytes::from_str("ICE")).unwrap();
  let ct_hex = ct.hex();
  println!("ct: {}", ct_hex);
  assert_eq!(ct_hex, String::from_str(expected));
}
