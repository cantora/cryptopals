pub mod mode {
pub mod cbc {
  extern crate openssl;
  use self::openssl::crypto::symm;
  use super::super::super::super::crypto;
  use padding::pkcs7;
  use bytes::Bytes;
  use std::iter;

  pub struct Context {
    crypter: symm::Crypter,
    dir: crypto::Direction,
    iv: Bytes
  }

  impl Context {
    pub fn new(crypter: symm::Crypter,
               dir: crypto::Direction,
               iv: Vec<u8>)
           -> Context {
      Context {
        crypter: crypter,
        dir: dir,
        iv: Bytes(iv)
      }
    }

    fn aes_128_base(dir: crypto::Direction, key: &[u8], iv: Vec<u8>)
       -> Context {
      assert_eq!(key.len(), 16);
      assert_eq!(iv.len(), 16);

      let crypter = symm::Crypter::new(symm::AES_128_ECB);
      let mode = match dir {
        crypto::Encrypt => symm::Encrypt,
        crypto::Decrypt => symm::Decrypt
      };

      crypter.init(mode, key, vec![]);
      crypter.pad(false);
      Context::new(crypter, dir, iv)
    }

    pub fn aes_128_enc(key: &[u8], iv: Vec<u8>) -> Context {
      Context::aes_128_base(crypto::Encrypt, key, iv)
    }

    pub fn aes_128_dec(key: &[u8], iv: Vec<u8>) -> Context {
      Context::aes_128_base(crypto::Decrypt, key, iv)
    }

    pub fn block_len(&self) -> uint {
      self.iv.len()
    }

    pub fn direction(&self) -> crypto::Direction {
      self.dir
    }

    fn process_enc(&mut self, data: &[u8]) -> Vec<u8> {
      let Bytes(input) = Bytes::from_slice(data) ^ self.iv;

      let output = self.crypter.update(input.as_slice());
      self.iv = Bytes(output.clone());

      output
    }

    fn process_dec(&mut self, data: &[u8]) -> Vec<u8> {
      let output = Bytes(self.crypter.update(data));
      let Bytes(dec) = output ^ self.iv;

      self.iv = Bytes::from_slice(data);

      dec
    }

    pub fn process(&mut self, data: &[u8]) -> Vec<u8> {
      assert_eq!(data.len(), self.block_len());
      match self.dir {
        crypto::Encrypt => self.process_enc(data),
        crypto::Decrypt => self.process_dec(data)
      }
    }
  } /* impl Context */

  pub struct DecStream {
    cbc: Context,
    buf: Vec<u8>
  }

  impl DecStream {
    pub fn new(cbc: Context) -> DecStream {
      DecStream { cbc: cbc, buf: Vec::new() }
    }

    pub fn aes_128_dec(key: &[u8], iv: Vec<u8>) -> DecStream {
      DecStream::new(Context::aes_128_dec(key, iv))
    }

    pub fn push(&mut self, b: u8) -> Option<Vec<u8>> {
      let blen = self.cbc.block_len();
      if self.buf.len() < blen {
        self.buf.push(b);
        None
      }
      else {
        let result = Some(self.cbc.process(self.buf.as_slice()));
        self.buf.clear();
        self.buf.push(b);
        result
      }      
    }

    pub fn finish(mut self) -> Result<Vec<u8>, Vec<u8>> {
      if self.buf.len() == self.cbc.block_len() {
        let mut result = self.cbc.process(self.buf.as_slice());
        match pkcs7::unpad(&mut result) {
          Some(_) => Err(self.buf),
          None    => Ok(result)
        }
      }
      else { /* padding error */
        Err(self.buf)
      }
    }

    pub fn iter<'a, T: iter::Iterator<&'a u8>>(&'a mut self,
                                               itr: &'a mut T)
           -> DecIter<'a, T> {
      DecIter::new(self, itr)
    }
  }

  pub struct DecIter<'a, T: 'a> {
    stm: &'a mut DecStream,
    itr: &'a mut T
  }

  impl<'a, T: iter::Iterator<&'a u8>> DecIter<'a, T> {
    pub fn new(stm: &'a mut DecStream, itr: &'a mut T)
           -> DecIter<'a, T> {
      DecIter { stm: stm, itr: itr }
    }
  }

  impl<'a, T: iter::Iterator<&'a u8>> 
      iter::Iterator<Vec<u8>>
      for DecIter<'a, T> {
    fn next(&mut self) -> Option<Vec<u8>> {
      loop {
        match self.itr.next() {
          Some(b) => {
            match self.stm.push(*b) {
              Some(result) => {
                return Some(result);
              }
              None         => { ; }
            }
          }
          None    => {
            return None;
          }
        } /* match itr next */
      } /* loop */
    } /* next */
  } /* impl Iterator */

} /* cbc */
} /* mode */
