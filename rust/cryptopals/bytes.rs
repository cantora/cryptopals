extern crate std;
extern crate serialize;

use self::serialize::base64;
use self::serialize::base64::ToBase64;
use self::serialize::hex::{FromHex, FromHexError};

use std::fmt;
use std::vec::Vec;
use std::path::BytesContainer;

use byte;
use byte::Byte;
use byte::ClassFlags;
use byte::Histogram;
use byte::NormalHistogram;

#[deriving(PartialEq, Eq)]
pub struct Bytes(pub Vec<u8>);

impl fmt::Show for Bytes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Bytes(ref vec) = *self;

    vec.iter().fold(Ok(()), |prev, b| {
      match prev {
        Ok(_) => write!(f, "{}", Byte(*b)),
        _ => prev
      }
    })
  }
}

impl BytesContainer for Bytes {
  fn container_as_bytes<'a>(&'a self) -> &'a [u8] {
    let Bytes(ref bvec) = *self;
    bvec.container_as_bytes()
  }
}

impl ToBase64 for Bytes {
  fn to_base64(&self, config: base64::Config) -> String {
    self.container_as_bytes().to_base64(config)
  }
}

macro_rules! maxmin_on_len(
  ($a:ident, $b:ident) => (
    if $a.len() < $b.len() {
      ($b, $a)
    }
    else {
      ($a, $b)
    }
  );
)

impl BitXor<Bytes, Bytes> for Bytes {
  fn bitxor(&self, rhs: &Bytes) -> Bytes {
    let Bytes(ref vec1) = *self;
    let Bytes(ref vec2) = *rhs;
    let (max_v, min_v) = maxmin_on_len!(vec1, vec2);

    if max_v.len() < 1 {
      return Bytes(Vec::new());
    }

    let mut out_v: Vec<u8> = Vec::with_capacity(max_v.len());

    for i in range(0, min_v.len()) {
      let b = *max_v.get(i) ^ *min_v.get(i);
      out_v.push(b);
    }
    for j in range(min_v.len(), max_v.len()) {
      out_v.push(*max_v.get(j));
    }

    return Bytes(out_v);
  }
}

impl Bytes {
  pub fn from_slice<'a>(bs: &'a [u8]) -> Bytes {
    Bytes(Vec::from_slice(bs))
  }

  pub fn from_hex_str(s: &str) -> Result<Bytes, FromHexError> {
    s.from_hex().and_then(|bvec| {
      Ok(Bytes(bvec))
    })
  }

  pub fn base64(&self) -> String {
    self.to_base64(base64::Config {
      char_set: base64::Standard,
      pad: true,
      line_length: None
    })
  }

  pub fn xor_byte(&self, rhs: u8) -> Bytes {
    let Bytes(ref vec1) = *self;

    if vec1.len() < 1 {
      return Bytes(Vec::new());
    }

    let mut out_v: Vec<u8> = Vec::with_capacity(vec1.len());

    for i in range(0, vec1.len()) {
      let b = *vec1.get(i) ^ rhs;
      out_v.push(b);
    }

    return Bytes(out_v);
  }

  pub fn has_byte_class(&self, flags: ClassFlags) -> bool {
    let Bytes(ref vec) = *self;

    for b in vec.iter() {
      if Byte(*b).class().intersects(flags) {
        return true;
      }
    }

    return false;
  }

  pub fn is_printable_ascii(&self) -> bool {
    !self.has_byte_class(byte::Control | byte::HighBit)
  }

  pub fn hist(&self) -> Histogram {
    let Bytes(ref vec) = *self;
    Histogram::from_iter(vec.iter())
  }

  pub fn normal_hist(&self) -> NormalHistogram {
    NormalHistogram::from_histogram(&self.hist())
  }
}
