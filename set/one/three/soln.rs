#![feature(phase)]
extern crate cryptopals;
#[phase(plugin, link)]
extern crate static_mdo;

use cryptopals::Bytes;
use cryptopals::byte;
use cryptopals::util::PQCell;
use cryptopals::PriorityQueue;

fn english_freqs(b: u8) -> f64 {
  match b {
    b'a' => 0.08167,
    b'b' => 0.01492,
    b'c' => 0.02782,
    b'd' => 0.04253,
    b'e' => 0.12702,
    b'f' => 0.02228,
    b'g' => 0.02015,
    b'h' => 0.06094,
    b'i' => 0.06966,
    b'j' => 0.00153,
    b'k' => 0.00772,
    b'l' => 0.04025,
    b'm' => 0.02406,
    b'n' => 0.06749,
    b'o' => 0.07507,
    b'p' => 0.01929,
    b'q' => 0.00095,
    b'r' => 0.05987,
    b's' => 0.06327,
    b't' => 0.09056,
    b'u' => 0.02758,
    b'v' => 0.00978,
    b'w' => 0.02360,
    b'x' => 0.00150,
    b'y' => 0.01974,
    b'z' => 0.00074,
    _    => 0.0
  }
}

fn main() {
  let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
  let engl_hist = byte::NormalHistogram::from_fn(english_freqs);

  println!("input: {}", input);

  let bs = Bytes::from_hex_str(input).unwrap();
  println!("bs: {}", bs);

  let mut pq: PriorityQueue<PQCell<f64, Bytes>> = PriorityQueue::new();
  for b in byte::all() {
    let xord = bs.xor_byte(b);
    if xord.is_printable_ascii() {
      let score = 1.0f64 - xord.normal_hist().distance_chi2(&engl_hist);
      pq.push(PQCell {
        priority: score,
        value: xord
      });
    }
  }

  for i in range(0i, 5) {
    match pq.pop() {
      Some(ref pqcell) => println!("{}: {}", i, pqcell.value),
      _                => break
    }
  }
}
