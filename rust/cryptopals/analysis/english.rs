extern crate std;
use std::iter;

use bytes::Bytes;
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

#[deriving(PartialEq)]
struct HeapByChi2Data<T> {
  pub data: Bytes,
  pub meta: T
}

pub struct HeapByChi2<T>(
  pub PriorityQueue<
        PQCell<
          f64,
          HeapByChi2Data<T>
        >
      >
);

impl<T: PartialEq> HeapByChi2<T> {

  pub fn new() -> HeapByChi2<T> {
    HeapByChi2(PriorityQueue::new())
  }

  pub fn add(&mut self, bs: Bytes, metadata: T) -> f64 {
    let score = -bs.normal_hist().distance_chi2(&histogram());
    let HeapByChi2(ref mut pq) = *self;

    pq.push(PQCell {
      priority: score,
      value: HeapByChi2Data {
        data: bs,
        meta: metadata
      }
    });

    score
  }

  pub fn pop(&mut self) -> Option<(f64, Bytes, T)> {
    let HeapByChi2(ref mut pq) = *self;

    match pq.pop() {
      Some(pqcell) =>
        Some(
          (pqcell.priority, pqcell.value.data, pqcell.value.meta)
        ),
      _            => None
    }
  }

  pub fn consume(self) -> HeapByChi2Iter<T> {
    HeapByChi2Iter(self)
  }
}


pub struct HeapByChi2Iter<T> (
  pub HeapByChi2<T>
);

impl<T: PartialEq> iter::Iterator<(f64, Bytes, T)>
                   for HeapByChi2Iter<T> {
  fn next(&mut self) -> Option<(f64, Bytes, T)> {
    let HeapByChi2Iter(ref mut hp) = *self;
    hp.pop()
  }
}

pub mod find {
pub mod key {
  use bytes::Bytes;
  pub fn best_xor<'a>(bs: &Bytes, keys: &'a [Bytes]) -> Result<(f64, Bytes, &'a Bytes), &'static str> {
    let mut engl_heap = super::super::HeapByChi2::new();
  
    for key in keys.iter() {
      match bs.xor_bytes(key) {
        Ok(xord) => { engl_heap.add(xord, key); },
        Err(e)   => return Err(e)
      }      
    }
  
    match engl_heap.pop() {
      Some(tpl) => Ok(tpl),
      None      => Err("keys was empty")
    }
  }
} /* mod key */  
} /* mod find */
