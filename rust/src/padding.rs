
#[derive(PartialEq)]
pub enum PaddingError {
  VectorTooLarge,
  InvalidPadding
}

pub mod pkcs7 {
use padding::PaddingError;

pub fn pad(bv: &mut Vec<u8>, block_sz: u8) -> u8 {
  let len: usize = bv.len();
  let k = block_sz;

  assert!(k > 0);

  //u8 fits in usize so no overflow worries
  let partial_len: u8 = (len % (k as usize)) as u8;
  //partial_len < k by definition of modulo, so no overflow worries
  let pad_amt         = k - partial_len;

  for _ in (0..pad_amt) {
    bv.push(pad_amt);
  }

  assert!(bv.len() % (k as usize) == 0);

  return pad_amt;
}

pub fn unpad(bv: &mut Vec<u8>) -> Option<PaddingError> {
  if bv.len() > 255 {
    return Some(PaddingError::VectorTooLarge);
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
      let plen_usize = amt as usize;
      let bv_len    = bv.len();
      assert!(plen_usize <= bv_len);
      bv.truncate(bv_len - plen_usize);
      None
    }
    None      => Some(PaddingError::InvalidPadding)
  }
}

} /* pkcs7 */
