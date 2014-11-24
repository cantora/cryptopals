use std::io;
use std::io::IoResult;
use std::io::IoErrorKind;
use std::io::standard_error;

pub struct Reader<T> {
  rdr: T,
  amt: uint,
  max_zeros: uint
}

#[deriving(Show,PartialEq,Eq)]
pub struct ReadStats {
  reads: uint,
  zero_reads: uint
}

pub fn new<T: io::Reader>(rdr: T, max_zeros: uint) -> Reader<T> {
  Reader::new(rdr, max_zeros)
}

impl<T: io::Reader> Reader<T> {
  pub fn new(rdr: T, max_zeros: uint) -> Reader<T> {
    Reader {
      rdr: rdr,
      amt: 0,
      max_zeros: max_zeros
    }
  }

  pub fn read(&mut self, buf: &mut [u8]) -> IoResult<ReadStats> {
    let min = buf.len();
    let mut stats = ReadStats {reads: 0, zero_reads: 0};
    let mut zeros = 0;

    while self.amt < min {
      loop {
        match self.rdr.read(buf.slice_from_mut(self.amt)) {
          Ok(0)   => {
            zeros += 1;
            stats.zero_reads += 1;
            if zeros >= self.max_zeros {
              return Err(standard_error(IoErrorKind::NoProgress))
            }
          }
          Ok(n)    => {
            zeros = 0;
            stats.reads += 1;
            self.amt += n;
            break; /* loop, continues in while */
          }
          Err(err)  => {
            return Err(err);
          }
        } /* match */
      } /* loop */
    } /* while */

    assert_eq!(self.amt, min);
    self.amt = 0;
    Ok(stats)
  }

  pub fn remain(&self) -> uint {
    self.amt
  }
}


