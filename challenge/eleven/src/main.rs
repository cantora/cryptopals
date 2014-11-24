#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin, link)]
extern crate cryptopals;
#[phase(plugin, link)]
extern crate static_mdo;

use std::io::File;
//use std::io::BufferedReader;
use cryptopals::Bytes;
use cryptopals::block;
use std::io::IoErrorKind;


fn main() {
  let f = File::open(&Path::new("words.txt")).unwrap();
  let mut buf = [0u8, ..128];

  let mut br = block::Reader::new(f, 1);
  let status = result_repeat!( br.read(&mut buf) => {
    println!("{}", Bytes::from_slice(buf.as_slice()));
  });

  result_err!( err <- status => {
    if err.kind != IoErrorKind::EndOfFile {
      panic!("failedasdf to read file: {}", err);
    }
  })

  println!("{}", Bytes::from_slice(buf.slice(0, br.remain())));
}
