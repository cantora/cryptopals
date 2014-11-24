use std::collections::TreeMap;
use std::fmt;
use std::num::Float;

pub struct Analyzer<T> {
  map: TreeMap<T, uint>,
}


impl<T: Ord> Analyzer<T> {
  pub fn new() -> Analyzer<T> {
    Analyzer {
      map: TreeMap::new()
    }
  }

  pub fn add(&mut self, symbol: T) -> uint {
    let new_count = match self.map.remove(&symbol) {
      Some(count) => count + 1,
      None        => 1u
    };
    self.map.insert(symbol, new_count);
    new_count
  }

  pub fn entropy(&self) -> f64 {
    let mut sum = 0.0f64;
    let sz = self.map.len() as f64;

    for count in self.map.values() {
      let prob_of_value = (*count as f64)/sz;
      sum += prob_of_value * prob_of_value.log2();
    }

    -sum
  }

}

impl<T: fmt::Show + Ord> fmt::Show for Analyzer<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.map.fmt(f)
  }
}

pub fn bits_per_symbol(alphabet_size: u64) -> f64 {
  alphabet_size.to_f64()
               .unwrap() /* its just some type of integer */
               .log2()   /* asserted above that its > 0 */
}


pub fn from_iter<T: Ord, U: Iterator<T>>(mut itr: U) -> f64 {
  let mut ea: Analyzer<T> = Analyzer::new();

  for symbol in itr {
    ea.add(symbol);
  }

  ea.entropy()
}
