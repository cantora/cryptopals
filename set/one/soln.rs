extern crate cryptopals;

use cryptopals::Bytes;

fn main() {
  let input = "49276d206b696c6c696e6720796f7572\
               20627261696e206c696b65206120706f\
               69736f6e6f7573206d757368726f6f6d";
  println!("hex: {}", input);

  match Bytes::from_hex_str(input) {
    Ok(bs) => {
      println!("ascii: {}", bs);
      println!("base64: {}", bs.base64());
    }
    Err(err) => {
      println!("invalid hex: {}", err);
    }
  }
}
