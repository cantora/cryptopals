extern crate std;

use std::fmt;

pub struct Byte(pub u8);

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
      0x20..0x7e => write!(f, "{:c}", Byte(b)),
      _          => write!(f, "\\x{:02x}", b)
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
