#![feature(macro_rules)] 
extern crate cryptopals;

use cryptopals::Bytes;

#[macro_export]
macro_rules! result_do(
    (
        let $p:path = $e:expr ; $( $t:tt )*
    ) => (
        { let $p = $e ; result_do! { $( $t )* } }
    );

    (
        $p:pat <- $e:expr ; $( $t:tt )*
    ) => (
        match $e {
          Ok($p)     => result_do! { $( $t )* },
          Err(err)   => Err(err)
        }
    );

    (
        ign $e:expr ; $( $t:tt )*
    ) => (
        match $e {
          _ => result_do! { $( $t )* }
        }
    );

    (
        ret $f:expr
    ) => (
        Ok($f)
    )
)

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
