use std::collections::BTreeMap;
use std::fmt;
use std::num::Float;

pub struct Analyzer<T> {
  map: BTreeMap<T, usize>,
}


impl<T: Ord> Analyzer<T> {
  pub fn new() -> Analyzer<T> {
    Analyzer {
      map: BTreeMap::new()
    }
  }

  pub fn add(&mut self, symbol: T) -> usize {
    let new_count = match self.map.remove(&symbol) {
      Some(count) => count + 1,
      None        => 1
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

impl<T: fmt::Debug + Ord> fmt::Debug for Analyzer<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.map.fmt(f)
  }
}

pub fn bits_per_symbol(alphabet_size: u64) -> f64 {
  alphabet_size.to_f64()
               .unwrap() /* its just some type of integer */
               .log2()   /* asserted above that its > 0 */
}


pub fn from_iter<U>(mut itr: U) -> f64 
       where U:       Iterator,
             U::Item: Ord      {
  let mut ea: Analyzer<U::Item> = Analyzer::new();

  for symbol in itr {
    ea.add(symbol);
  }

  ea.entropy()
}
