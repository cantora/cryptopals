use std::iter::Iterator;

//use std::io::IoResult;

/*******************************************
 * Modulo iterator
 * ---------------
 * iterate, returning every nth item.
 */
pub struct Modulo<U> {
  idx: uint,
  modulus: uint,
  iter: U
}

impl<U> Modulo<U> {
  pub fn new(modulus: uint, iter: U) -> Modulo<U> {
    assert!(modulus > 0);

    Modulo {
      idx: modulus,
      modulus: modulus,
      iter: iter
    }
  }
}

impl<T, U: Iterator<T>> Iterator<T> for Modulo<U> {
  fn next(&mut self) -> Option<T> {
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
  idx: uint,
  modulus: uint,
  v: &'a Vec<T>
}

impl<'a, T> Transposed<'a, T> {
  pub fn new(vec: &'a Vec<T>, modulus: uint) -> Transposed<'a, T> {
    assert!(modulus > 1);
    Transposed {
      idx: 0,
      modulus: modulus,
      v: vec
    }
  }
}

impl<'a, T: Clone> Iterator<Vec<T>> for Transposed<'a, T> {
  fn next(&mut self) -> Option<Vec<T>> {
    let mut classes = self.modulus;

    if classes > self.v.len() {
      classes = self.v.len()
    }

    if self.idx >= classes {
      return None;
    }

    let mut mod_iter = Modulo::new(
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

/*******************************************
 * block reader
 * ---------------
 * iterate of blocks from buffered reader
 */

/*
struct BlockReader<'a, T> (pub T, pub &'a mut [u8]);

impl<'a, T: Reader> Iterator<IoResult<uint>> for BlockReader<'a, T> {
  fn next(&mut self) -> Option<IoResult<uint>> {
    let BlockReader(ref mut rdr, ref mut buf) = *self;

    match rdr.read(*buf) {
      Ok(amt) => Some(Ok(amt)),
      Err(_)  => None
    }
  }
}
*/