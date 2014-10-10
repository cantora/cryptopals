extern crate std;
use std::iter;

use bytes::Bytes;
use pqueue;
use pqueue::PriorityQueue;
use util::PQCell;
use byte::NormalHistogram;

fn freqs(b: u8) -> f64 {
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

static mut HISTOGRAM: Option<NormalHistogram> = None;

pub fn histogram() -> NormalHistogram {
  unsafe {
  match HISTOGRAM {
    Some(h) => h,
    None    => {
      let h = NormalHistogram::from_fn(freqs);
      HISTOGRAM = Some(h);
      h
    }
  }}
}

pub struct HeapByChi2(
  pub PriorityQueue<PQCell<f64, Bytes>>
);

impl HeapByChi2 {

  pub fn new() -> HeapByChi2 {
    HeapByChi2(PriorityQueue::new())
  }

  pub fn add(&mut self, bs: Bytes) -> f64 {
    let score = -bs.normal_hist().distance_chi2(&histogram());
    let HeapByChi2(ref mut pq) = *self;

    pq.push(PQCell {
      priority: score,
      value: bs
    });

    score
  }

  pub fn pop(&mut self) -> Option<(f64, Bytes)> {
    let HeapByChi2(ref mut pq) = *self;

    match pq.pop() {
      Some(pqcell) => Some((pqcell.priority, pqcell.value)),
      _            => None
    }
  }

  pub fn iter<'a>(&'a self) -> HeapByChi2Iter<'a> {
    let HeapByChi2(ref pq) = *self;
    HeapByChi2Iter(pq.iter())
  }
}

pub struct HeapByChi2Iter<'a> (
  pub pqueue::Items<'a,PQCell<f64,Bytes>>
);

impl<'a> iter::Iterator<(f64, &'a Bytes)> for HeapByChi2Iter<'a> {
  fn next(&mut self) -> Option<(f64, &'a Bytes)> {
    let HeapByChi2Iter(ref mut itr) = *self;
    match itr.next() {
      Some(pqcell) => Some((pqcell.priority, &pqcell.value)),
      _            => None
    }
  }
}
