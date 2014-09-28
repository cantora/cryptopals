extern crate std;
extern crate serialize;

use self::serialize::base64;
use self::serialize::base64::ToBase64;
use self::serialize::hex::{FromHex, FromHexError};

use std::fmt;
use std::path::BytesContainer;

use byte::Byte;

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

}
