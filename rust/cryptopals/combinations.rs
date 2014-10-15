use std::iter;

pub struct Range {
  primary: iter::Range<uint>,
  secondary: iter::Range<uint>,
  current_x: uint,
  size: uint
}

impl Range {
  pub fn new(start: uint, size: uint) -> Range {
    assert!(size > 1u);
    let mut itr = iter::range(start, start+size-1);
    let current = itr.next().unwrap();
    Range {
      primary: itr,
      secondary: iter::range(current+1, start+size),
      current_x: current,
      size: size
    }
  }
}

impl iter::Iterator<(uint, uint)> for Range {
  fn next(&mut self) -> Option<(uint, uint)> {
    match self.secondary.next() {
      Some(y) => Some((self.current_x, y)),
      None    => {
        match self.primary.next() {
          Some(x) => {
            self.current_x = x;
            self.secondary = iter::range(self.current_x+1, self.size);
            Some((self.current_x, self.secondary.next().unwrap()))
          },
          None    => None
        }
      }
    }
  }
}
