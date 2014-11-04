
pub fn pkcs7(bv: &mut Vec<u8>, block_sz: u8) -> u8 {
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
