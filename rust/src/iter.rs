use std::iter::Iterator;

//TODO: use generic unsigned int for these
/*******************************************
 * Modulo iterator
 * ---------------
 * iterate, returning every nth item.
 */
pub struct Modulo<U> {
  idx: usize,
  modulus: usize,
  iter: U
}

impl<U> Modulo<U> {
  pub fn new(modulus: usize, iter: U) -> Modulo<U> {
    assert!(modulus > 0);

    Modulo {
      idx: modulus,
      modulus: modulus,
      iter: iter
    }
  }
}

impl<U: Iterator> Iterator for Modulo<U> {
  type Item = U::Item;

  fn next(&mut self) -> Option<U::Item> {
    while self.idx < self.modulus {
      if self.iter.next().is_none() {
        return None;
      }
      self.idx += 1;
    }

    self.idx = 1;
    self.iter.next()
  }
}

/*******************************************
 * Transposed iterator
 * ---------------
 * iterate over the elements of a transposed vector.
 * example:
 * abcdefghijklmnopqrstuvwxyz
 *           ||
 *           \/
 * "adgjmpsvy", "behknqtwz", "cfilorux"
 */

pub struct Transposed<'a, T: 'a> {
  idx: usize,
  modulus: usize,
  v: &'a Vec<T>
}

impl<'a, T> Transposed<'a, T> {
  pub fn new(vec: &'a Vec<T>, modulus: usize) -> Transposed<'a, T> {
    assert!(modulus > 1);
    Transposed {
      idx: 0,
      modulus: modulus,
      v: vec
    }
  }
}

impl<'a, T: Clone> Iterator for Transposed<'a, T> {
  type Item = Vec<T>;

  fn next(&mut self) -> Option<Vec<T>> {
    let mut classes = self.modulus;

    if classes > self.v.len() {
      classes = self.v.len()
    }

    if self.idx >= classes {
      return None;
    }

    let mod_iter = Modulo::new(
      self.modulus,
      self.v.iter().skip(self.idx)
    );

    let mut new_v = Vec::new();
    for x in mod_iter {
      new_v.push(x.clone())
    }
    self.idx += 1;

    Some(new_v)
  }
}
