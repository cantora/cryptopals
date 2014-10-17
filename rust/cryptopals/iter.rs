use std::iter;

pub struct Modulo<U> {
  idx: uint,
  modulus: uint,
  iter: U
}

impl<U> Modulo<U> {
  pub fn new(modulus: uint, iter: U) -> Modulo<U> {
    assert!(modulus > 0);

    Modulo {
      idx: 0,
      modulus: modulus,
      iter: iter
    }
  }
}

impl<T, U: iter::Iterator<T>> iter::Iterator<T> for Modulo<U> {
  fn next(&mut self) -> Option<T> {
    while self.idx % self.modulus != 0 {
      if self.iter.next() == None {
        return None;
      }
      self.idx += 1;
    }

    self.idx = 1;
    self.iter.next()
  }
}
