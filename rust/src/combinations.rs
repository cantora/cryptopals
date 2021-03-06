extern crate rand;

use std::iter;
use std::collections::BTreeSet;
use std::vec;
use std::ops;

//TODO: convert this stuff to generic unsigned int
pub struct Range {
  primary: ops::Range<usize>,
  secondary: ops::Range<usize>,
  current_x: usize,
  end: usize
}

/* represents a tuple combination iterator
 * over the range start .. start + size. the
 * number of items in the range will be
 * (size choose 2), so the minimum size is 2.
 */
impl Range {
  pub fn new(start: usize, size: usize) -> Range {
    assert!(size > 1);
    let end = start+size;
    //len(itr) = size-1 >= 1
    let mut itr = start..end-1;
    let current = itr.next().unwrap();
    Range {
      primary: itr,
      secondary: current+1..end,
      current_x: current,
      end: end
    }
  }
}

impl iter::Iterator for Range {
  type Item = (usize, usize);

  fn next(&mut self) -> Option<(usize, usize)> {
    match self.secondary.next() {
      Some(y) => Some((self.current_x, y)),
      None    => {
        match self.primary.next() {
          Some(x) => {
            self.current_x = x;
            self.secondary = self.current_x+1..self.end;
            Some((self.current_x,
                  self.secondary.next().unwrap()))
          },
          None    => None
        }
      }
    }
  }
}

fn _n_choose_k(n: usize, k: usize) -> usize {
  match k {
    0 => 1,
    _ => (n * n_choose_k(n - 1, k - 1)) / k
  }
}

pub fn n_choose_k(n: usize, k: usize) -> usize {
  assert!(n > 0);
  _n_choose_k(n, k)
}

pub struct Random {
  base: usize,
  sz: usize,
  history: BTreeSet<(usize,usize)>,
  max: usize
}

impl Random {
  pub fn new(start: usize, size: usize) -> Random {
    assert!(size > 1);
    /* whats the idiomatic way to check for overflow in rust? */
    Random {
      base: start,
      sz: size,
      history: BTreeSet::new(),
      max: n_choose_k(size, 2)
    }
  }

  pub fn random_pair(&self) -> (usize, usize) {
    let a = rand::random::<usize>() % self.sz;
    let mut b = rand::random::<usize>() % self.sz;
    while b == a {
      b = rand::random::<usize>() % self.sz;
    }

    (self.base + a, self.base + b)
  }
}

impl iter::Iterator for Random {
  type Item = (usize, usize);

  fn next(&mut self) -> Option<(usize, usize)> {
    if self.history.len() >= self.max {
      return None;
    }

    let mut tpl = self.random_pair();
    while self.history.contains(&tpl) {
      tpl = self.random_pair();
    }

    Some(tpl)
  }
}

pub struct RandomSlice<'a, T: 'a> {
  modulus: usize,
  vec: &'a vec::Vec<T>,
  rnd: Random
}

impl<'a, T> RandomSlice<'a, T> {
  pub fn new(modulus: usize, vec: &'a vec::Vec<T>) -> RandomSlice<'a, T> {
    assert!(modulus > 1);
    let vlen = vec.len();
    assert!(vlen > 2*modulus);

    let random_sz = vlen/modulus;
    assert!(random_sz >= 2);

    RandomSlice {
      modulus: modulus,
      vec: vec,
      rnd: Random::new(0, random_sz)
    }
  }
}

impl<'a, T> iter::Iterator for RandomSlice<'a, T> {
  type Item = (&'a [T], &'a [T]);

  fn next(&mut self) -> Option<(&'a [T], &'a [T])> {
    match self.rnd.next() {
      Some((a, b)) => {
        let start_a = a*self.modulus;
        let start_b = b*self.modulus;
        let end_a   = start_a + self.modulus;
        let end_b   = start_b + self.modulus;
        let sl_a    = &self.vec[start_a..end_a];
        let sl_b    = &self.vec[start_b..end_b];
        Some((sl_a, sl_b))
      },
      _            => None
    }
  }
}

