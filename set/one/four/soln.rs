#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin, link)]
extern crate cryptopals;
#[phase(plugin, link)]
extern crate static_mdo;

use std::io::File;
use std::io::BufferedReader;

use cryptopals::Bytes;
use cryptopals::byte;
use cryptopals::analysis::english;

fn top_engl_bs(bs: Bytes) -> (f64, Bytes, u8) {
  let mut engl_heap = english::HeapByChi2::new();

  for b in byte::all() {
    engl_heap.add(bs.xor_byte(b), b);
  }

  engl_heap.pop().unwrap()
}

fn main() {
  let mut file = BufferedReader::new(File::open(&Path::new("4.txt")));
  let mut i = 0u;
  let mut high_score = -1.0f64;
  let mut winner = Bytes::new();
  let mut winner_key = 0u8;
  let mut winner_str = String::new();

  let status = result_for!( line in file.lines() {
    let s = line.as_slice().trim();

    match Bytes::from_hex(&s) {
      Ok(bs)   => {
        let (score, dec, key) = top_engl_bs(bs);
        if score > high_score {
          high_score = score;
          winner = dec;
          winner_key = key;
          winner_str = String::from_str(s);
        }
      }
      Err(err) => {
        errln!("could not decode from hex string {}: {}", s, err);
      }
    }

    err!("processed line {}\r", i);
    i += 1;
  });
  println!("");

  if status.is_some() {
    errln!("IO error terminated loop: {}", status.unwrap());
  }

  println!("high score: {}", high_score);
  println!("winning string (key = {}): {}",
           winner_key, winner_str);
  println!("plain text: {}", winner);
}
