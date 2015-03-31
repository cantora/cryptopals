extern crate std;
extern crate rustc_serialize as serialize;
extern crate rand;

use self::serialize::base64;
use self::serialize::base64::{ToBase64,FromBase64,FromBase64Error};
use self::serialize::hex;
use self::serialize::hex::ToHex;
use self::serialize::hex::FromHex;

use std::fmt;
use std::vec::Vec;
use std::slice;
//use std::path::BytesContainer;
use std::iter::Skip;
use std::ops::BitXor;

use byte;
use byte::Byte;
use byte::ClassFlags;
use byte::Histogram;
use byte::NormalHistogram;

use iter::Modulo;
use iter::Transposed;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes(pub Vec<u8>);

impl fmt::Display for Bytes {
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

//impl BytesContainer for Bytes {
//  fn container_as_bytes<'a>(&'a self) -> &'a [u8] {
//    let Bytes(ref bvec) = *self;
//    bvec.container_as_bytes()
//  }
//}

impl ToBase64 for Bytes {
  fn to_base64(&self, config: base64::Config) -> String {
    let Bytes(ref vec) = *self;
    vec.to_base64(config)
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
);

impl<'a, 'b> BitXor<&'a Bytes> for &'b Bytes {
  type Output = Bytes;

  fn bitxor(self, rhs: &Bytes) -> Bytes {
    let Bytes(ref vec1) = *self;
    let Bytes(ref vec2) = *rhs;
    let (max_v, min_v) = maxmin_on_len!(vec1, vec2);

    if max_v.len() < 1 {
      return Bytes(Vec::new());
    }

    let mut out_v: Vec<u8> = Vec::with_capacity(max_v.len());

    for i in (0..min_v.len()) {
      let b = max_v[i] ^ min_v[i];
      out_v.push(b);
    }
    for j in (min_v.len()..max_v.len()) {
      out_v.push(max_v[j]);
    }

    return Bytes(out_v);
  }
}

pub type FromHexResult = Result<Bytes, hex::FromHexError>;
pub type FromBase64Result = Result<Bytes, FromBase64Error>;

impl hex::ToHex for Bytes {
  fn to_hex(&self) -> String {
    let Bytes(ref vec) = *self;
    vec.to_hex()
  }
}

impl AsRef<[u8]> for Bytes {
  fn as_ref<'a>(&'a self) -> &'a [u8] {
    let Bytes(ref v) = *self;
    &v[..]
  }
}

impl Bytes {
  pub fn new() -> Bytes {
    Bytes(Vec::new())
  }

  pub fn from_byte(b: u8) -> Bytes {
    Bytes::from_slice(&[b])
  }

  pub fn from_slice<'a>(bs: &'a [u8]) -> Bytes {
    Bytes(bs.to_vec())
  }

  pub fn from_str<'a>(bs: &'a str) -> Bytes {
    Bytes::from_slice(bs.as_bytes())
  }

  pub fn from_hex<T>(input: T) -> FromHexResult 
      where T: hex::FromHex {
    input.from_hex().and_then(|bvec| {
      Ok(Bytes(bvec))
    })
  }

  pub fn from_hex_string(s: &String) -> FromHexResult {
    s.from_hex().and_then(|bvec| {
      Ok(Bytes(bvec))
    })
  }

  pub fn from_base64<T: FromBase64>(s: &T) -> FromBase64Result {
    match s.from_base64() {
      Ok(bvec) => Ok(Bytes(bvec)),
      Err(err) => Err(err)
    }
  }

  pub fn random(len: usize) -> Bytes {
    if len < 1 {
      return Bytes::new();
    }

    let mut v: Vec<u8> = Vec::new();
    for _ in (0..len) {
      v.push(rand::random());
    }

    Bytes(v)
  }

  pub fn len(&self) -> usize {
    let Bytes(ref v) = *self;
    v.len()
  }

  pub fn mut_vec<'a>(&'a mut self) -> &'a mut Vec<u8> {
    let Bytes(ref mut bv) = *self;
    bv
  }

  pub fn vec<'a>(&'a self) -> &'a Vec<u8> {
    let Bytes(ref bv) = *self;
    bv
  }

  pub fn base64(&self) -> String {
    self.to_base64(base64::Config {
      char_set: base64::Standard,
      pad: true,
      line_length: None,
      newline: base64::Newline::LF,
    })
  }

  pub fn hex(&self) -> String {
    self.to_hex()
  }

  pub fn xor_byte(&self, rhs: u8) -> Bytes {
    self.xor_bytes(&Bytes::from_slice(&[rhs])).unwrap()
  }

  pub fn xor_bytes(&self, rhs: &Bytes)
         -> Result<Bytes, &'static str> {
    let Bytes(ref vec1) = *self;
    let Bytes(ref vec_rhs) = *rhs;
    let modulo = vec_rhs.len();

    if vec1.len() < 1 {
      return Ok(Bytes(Vec::new()));
    }
    if modulo < 1 {
      return Err("empty rhs");
    }

    let mut out_v: Vec<u8> = Vec::with_capacity(vec1.len());

    for i in (0..vec1.len()) {
      let b = vec1[i] ^ vec_rhs[i%modulo];
      out_v.push(b);
    }

    Ok(Bytes(out_v))
  }

  pub fn n_diff_bits(&self, rhs: &Bytes) -> Result<u32, &str> {
    let Bytes(ref vec1) = *self;
    let Bytes(ref vec2) = *rhs;
    let len = vec1.len();

    if len != vec2.len() {
      return Err("both byte sequences must be the same length");
    }

    if len < 1 {
      return Err("self.len() == 0");
    }

    Ok((self ^ rhs).n_set_bits())
  }

  pub fn hamming_distance(&self, rhs: &Bytes) -> Result<f64, &str> {
    match self.n_diff_bits(rhs) {
      Ok(n)          => {
        let sz = self.len()*8;
        Ok((n as f64)/(sz as f64))
      },
      Err(e)         => Err(e)                
    }
  }

  pub fn n_set_bits(&self) -> u32 {
    let Bytes(ref vec1) = *self;
    let mut count = 0u32;
    let bit_count_table: [u8; 16] = [
      0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4
    ];
  
  
    for b in vec1.iter() {
      count += bit_count_table[(*b & 0x0f) as usize] as u32;
      count += bit_count_table[(*b >> 4) as usize] as u32;
    }

    count
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
    !self.has_byte_class(byte::CONTROL | byte::HIGHBIT)
  }

  pub fn hist(&self) -> Histogram {
    let Bytes(ref vec) = *self;
    Histogram::from_iter(vec.iter())
  }

  pub fn normal_hist(&self) -> NormalHistogram {
    NormalHistogram::from_histogram(&self.hist())
  }

  pub fn transposed_n(&self, modulus: usize, n: usize)
         -> Modulo<Skip<slice::Iter<u8>>> {
    let Bytes(ref vec) = *self;
    Modulo::new(modulus, vec.iter().skip(n))
  }

  pub fn transposed(&self, modulus: usize) -> Transposed<u8> {
    let Bytes(ref vec) = *self;
    Transposed::new(vec, modulus)
  }
}
