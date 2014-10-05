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

bitflags!(
  flags ClassFlags: uint {
    static WhiteSpace    = 0x01,
    static Alphabetic    = 0x02,
    static Numeric       = 0x04,
    static Control       = 0x08,
    static Punctuation   = 0x10,
    static HighBit       = 0x20
  }
)

impl Byte {
  pub fn class(&self) -> ClassFlags {
    let Byte(b) = *self;

    match b {
      0x00..0x08     => Control,
      0x09..0x0b     => WhiteSpace,
      0x0c           => Control,
      0x0d           => WhiteSpace,
      0x0e..0x1f     => Control,
      0x20           => WhiteSpace,
      0x21..0x2f     => Punctuation,
      0x30..0x39     => Numeric,
      0x3a..0x40     => Punctuation,
      0x41..0x5a     => Alphabetic,
      0x5b..0x60     => Punctuation,
      0x61..0x7a     => Alphabetic,
      0x7b..0x7e     => Punctuation,
      0x7f           => Control,
      _              => HighBit
    }
  }
}

pub fn all() -> std::iter::RangeInclusive<u8> {
  std::iter::range_inclusive(0u8, 255)
}
