pub mod mode {
pub mod cbc {
  extern crate openssl;
  use self::openssl::crypto::symm;
  use super::super::super::super::crypto::Direction;
  use padding::pkcs7;
  use bytes::Bytes;
  use std::iter;

  pub struct Context {
    crypter: symm::Crypter,
    dir: Direction,
    iv: Bytes
  }

  impl Context {
    pub fn new(crypter: symm::Crypter,
               dir: Direction,
               iv: Vec<u8>)
           -> Context {
      Context {
        crypter: crypter,
        dir: dir,
        iv: Bytes(iv)
      }
    }

    fn aes_128_base(dir: Direction, key: &[u8], iv: Vec<u8>)
       -> Context {
      assert_eq!(key.len(), 16);
      assert_eq!(iv.len(), 16);

      let crypter = symm::Crypter::new(symm::Type::AES_128_ECB);
      let mode = match dir {
        Direction::Encrypt => symm::Mode::Encrypt,
        Direction::Decrypt => symm::Mode::Decrypt
      };

      crypter.init(mode, key, vec![]);
      crypter.pad(false);
      Context::new(crypter, dir, iv)
    }

    pub fn aes_128_enc(key: &[u8], iv: Vec<u8>) -> Context {
      Context::aes_128_base(Direction::Encrypt, key, iv)
    }

    pub fn aes_128_dec(key: &[u8], iv: Vec<u8>) -> Context {
      Context::aes_128_base(Direction::Decrypt, key, iv)
    }

    pub fn block_len(&self) -> usize {
      self.iv.len()
    }

    pub fn direction(&self) -> Direction {
      self.dir
    }

    fn process_enc(&mut self, data: &[u8]) -> Vec<u8> {
      let bs_data      = Bytes::from_slice(data);
      let Bytes(input) = &bs_data ^ &self.iv;

      let output = self.crypter.update(input.as_slice());
      self.iv = Bytes(output.clone());

      output
    }

    fn process_dec(&mut self, data: &[u8]) -> Vec<u8> {
      let output = Bytes(self.crypter.update(data));
      let Bytes(dec) = &output ^ &self.iv;

      self.iv = Bytes::from_slice(data);

      dec
    }

    pub fn process(&mut self, data: &[u8]) -> Vec<u8> {
      assert_eq!(data.len(), self.block_len());
      match self.dir {
        Direction::Encrypt => self.process_enc(data),
        Direction::Decrypt => self.process_dec(data)
      }
    }
  } /* impl Context */

  pub trait Stream {
    fn push(&mut self, b: u8) -> Option<Vec<u8>>;
    fn iter<'a, T: iter::Iterator>(&'a mut self,
                                   itr: &'a mut T)
       -> Iterator<'a, T, Self> {
      Iterator::new(self, itr)
    }
  }

  pub struct EncStream {
    cbc: Context,
    buf: Vec<u8>
  }

  impl Stream for EncStream {
    fn push(&mut self, b: u8) -> Option<Vec<u8>> {
      self.buf.push(b);
      if self.buf.len() < self.cbc.block_len() {
        return None;
      }

      let result = Some(self.cbc.process(self.buf.as_slice()));
      self.buf.clear();

      result
    }
  }

  impl EncStream {
    pub fn new(cbc: Context) -> EncStream {
      EncStream { cbc: cbc, buf: Vec::new() }
    }

    pub fn aes_128_enc(key: &[u8], iv: Vec<u8>) -> EncStream {
      EncStream::new(Context::aes_128_enc(key, iv))
    }

    pub fn finish(mut self) -> Vec<u8> {
      let blen = self.cbc.block_len();
      assert!(blen < 256);
      pkcs7::pad(&mut self.buf, blen as u8);
      println!("padded block: {}", Bytes(self.buf.clone()));
      self.cbc.process(self.buf.as_slice())
    }
  }

  pub struct DecStream {
    cbc: Context,
    buf: Vec<u8>
  }

  impl Stream for DecStream {
    fn push(&mut self, b: u8) -> Option<Vec<u8>> {
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
  }

  impl DecStream {
    pub fn new(cbc: Context) -> DecStream {
      DecStream { cbc: cbc, buf: Vec::new() }
    }

    pub fn aes_128_dec(key: &[u8], iv: Vec<u8>) -> DecStream {
      DecStream::new(Context::aes_128_dec(key, iv))
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
  }

  pub struct Iterator<'a, T: 'a + ?Sized, U: 'a + ?Sized> {
    stm: &'a mut U,
    itr: &'a mut T
  }

  impl<'a, T: iter::Iterator + ?Sized, U: Stream + ?Sized> Iterator<'a, T, U> {
    pub fn new(stm: &'a mut U, itr: &'a mut T)
           -> Iterator<'a, T, U> {
      Iterator { stm: stm, itr: itr }
    }
  }

  impl<'a, T: iter::Iterator<Item=&'a u8>, U: Stream> iter::Iterator for Iterator<'a, T, U> {
    type Item = Vec<u8>;

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
