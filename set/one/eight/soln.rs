#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin, link)]
extern crate cryptopals;
#[phase(plugin, link)]
extern crate static_mdo;

use std::io::File;
use std::io::BufferedReader;

use cryptopals::Bytes;
use cryptopals::entropy;

fn process(best_line: &mut Option<(Bytes, uint, f64)>,
           bs: Bytes,
           line: uint,
           block_sz: uint) {
  let entropy = entropy::from_iter(
    bs.as_slice().chunks(block_sz)
  );

  /*println!("entropy: {}", entropy);*/
  match *best_line {
    Some((_, _, e)) => {
      if entropy < e {
        *best_line = Some((bs, line, entropy));
      }
    }
    None         => {
      *best_line = Some((bs, line, entropy));
    }
  }
}

fn main() {
  let mut file = BufferedReader::new(File::open(&Path::new("8.txt")));
  let block_sz = 16u;
  let mut best_line: Option<(Bytes, uint, f64)> = None;
  let mut i = 0u;

  let status = result_for!( line in file.lines() {
    match Bytes::from_hex(&line.as_slice()) {
      Ok(bs)   => {
        process(&mut best_line, bs, i, block_sz);
      },
      Err(err) => {
        errln!("could not decode from hex string {}: {}", line, err);
      }
    }

    i += 1;
  });

  if status.is_some() {
    errln!("IO error terminated loop: {}", status.unwrap());
  }

  match best_line {
    Some((bs, line, entropy)) => {
      println!("best line was {} with score {}", line, entropy);
      for chunk in bs.as_slice().chunks(block_sz) {
        println!("  {}", Bytes::from_slice(chunk));
      }
    }
    None                => {
      errln!("empty file.");
    }
  }
}
