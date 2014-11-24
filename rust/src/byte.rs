extern crate std;

use std::fmt;
use std::rand;
use std::num::Float;

pub struct Byte(pub u8);

impl rand::Rand for Byte {
  fn rand<R: rand::Rng>(rng: &mut R) -> Byte {
    let b: u8 = rand::Rand::rand(rng);
    Byte(b)
  }
}

pub fn random() -> Byte {
  rand::random()
}

impl fmt::Char for Byte {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Byte(b) = *self;
    let ascii = b.to_ascii();
    ascii.to_char().fmt(f)
  }
}

impl fmt::Show for Byte {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Byte(b) = *self;

    match b {
      0x5c        => write!(f, "\\\\"),
      0x20...0x7e => write!(f, "{:c}", Byte(b)),
      _           => write!(f, "\\x{:02x}", b)
    }
  }
}

impl BitXor<Byte, Byte> for Byte {
  fn bitxor(&self, rhs: &Byte) -> Byte {
    let Byte(other_b) = *rhs;
    let Byte(b) = *self;

    Byte(b^other_b)
  }
}

bitflags!(
  flags ClassFlags: uint {
    const WHITESPACE    = 0x01,
    const ALPHABETIC    = 0x02,
    const NUMERIC       = 0x04,
    const CONTROL       = 0x08,
    const PUNCTUATION   = 0x10,
    const HIGHBIT       = 0x20
  }
)

impl Byte {
  pub fn class(&self) -> ClassFlags {
    let Byte(b) = *self;

    match b {
      0x00...0x08     => CONTROL,
      0x09...0x0b     => WHITESPACE,
      0x0c            => CONTROL,
      0x0d            => WHITESPACE,
      0x0e...0x1f     => CONTROL,
      0x20            => WHITESPACE,
      0x21...0x2f     => PUNCTUATION,
      0x30...0x39     => NUMERIC,
      0x3a...0x40     => PUNCTUATION,
      0x41...0x5a     => ALPHABETIC,
      0x5b...0x60     => PUNCTUATION,
      0x61...0x7a     => ALPHABETIC,
      0x7b...0x7e     => PUNCTUATION,
      0x7f            => CONTROL,
      _               => HIGHBIT
    }
  }
}

pub fn all() -> std::iter::RangeInclusive<u8> {
  std::iter::range_inclusive(0u8, 255)
}

#[deriving(Eq)]
pub struct Histogram {
  data: [uint, .. 256],
  total: uint
}

impl<'a> FromIterator<&'a u8> for Histogram {
  fn from_iter<T: Iterator<&'a u8>>(mut iterator: T) -> Histogram {
    let mut arr = [0u, .. 256];
    let mut count = 0u;

    for b in iterator {
      arr[*b as uint] += 1;
      count += 1;
    }

    Histogram { data: arr, total: count }
  }
}

impl PartialEq for Histogram {
  fn eq(&self, other: &Histogram) -> bool {
    if self.total != other.total {
      return false;
    }

    self.data.as_slice() != other.data.as_slice()
  }

  fn ne(&self, other: &Histogram) -> bool { !self.eq(other) }
}

impl fmt::Show for Histogram {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "total: {}\n", self.total));
    
    for b in all() {
      let count = self.data[b as uint];
      if count > 0 {
        try!(write!(f, " '{}' => {}\n", Byte(b), count));
      }
    }

    Ok(())
  }
}

impl Histogram {
  pub fn from_iter<'a, T: Iterator<&'a u8>>(iterator: T) -> Histogram {
    FromIterator::from_iter(iterator)
  }
}


pub struct NormalHistogram {
  data: [f64, .. 256]
}

impl NormalHistogram {
  pub fn from_histogram(hist: &Histogram) -> NormalHistogram {
    let mut arr = [0.0f64, .. 256];
    if hist.total < 1 {
      return NormalHistogram { data: arr };
    }

    let float_total = hist.total as f64;
    for b in all() {
      let idx = b as uint;
      arr[idx] = (hist.data[idx] as f64)/float_total;
    }

    NormalHistogram { data: arr }
  }

  pub fn from_fn(f: |u8| -> f64) -> NormalHistogram {
    let mut arr = [0.0f64, .. 256];
    for b in all() {
      arr[b as uint] = f(b);
    }

    NormalHistogram { data: arr }
  }

  pub fn distance_chi2(&self, other: &NormalHistogram) -> f64 {
    let mut result = 0.0f64;

    for b in all() {
      let idx = b as uint;
      let x_i = self.data[idx];
      let y_i = other.data[idx];
      let denom = x_i + y_i;
      if denom == 0.0 {
        continue;
      }

      result += (x_i - y_i).powi(2) / denom;
    }

    result * 0.5
  }
}
