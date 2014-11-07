
#[deriving(PartialEq, Show)]
pub enum PaddingError {
  VectorTooLarge,
  InvalidPadding
}

pub mod pkcs7 {
use padding;
use padding::PaddingError;

pub fn pad(bv: &mut Vec<u8>, block_sz: u8) -> u8 {
  let len: uint = bv.len();
  let k = block_sz;

  assert!(k > 0);

  let partial_len = len % (k as uint);
  let pad_amt     = k - (partial_len as u8);

  for _ in range(0, pad_amt) {
    bv.push(pad_amt);
  }

  assert!(bv.len() % (k as uint) == 0);

  return pad_amt;
}

pub fn unpad(bv: &mut Vec<u8>) -> Option<PaddingError> {
  if bv.len() > 255 {
    return Some(padding::VectorTooLarge);
  }

  let mut plen: Option<u8> = None;
  let mut i = bv.len() as u8;

  for &b in bv.iter() {
    plen = if b == i {
      Some(b)
    }
    else {
      match plen {
        Some(amt) if amt == b => Some(amt),
        _                     => None
      }
    };

    i -= 1;
  }

  match plen {
    Some(amt) => {
      let plen_uint = amt as uint;
      let bv_len    = bv.len();
      assert!(plen_uint <= bv_len);
      bv.truncate(bv_len - plen_uint);
      None
    }
    None      => Some(padding::InvalidPadding)
  }
}

} /* pkcs7 */
