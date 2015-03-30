use std::io;
use std::io::ErrorKind;

pub struct Reader<T> {
  rdr: T,
  amt: usize,
  max_zeros: usize
}

#[derive(PartialEq,Eq)]
pub struct ReadStats {
  reads: usize,
  zero_reads: usize
}

pub fn new<T: io::Read>(rdr: T, max_zeros: usize) -> Reader<T> {
  Reader::new(rdr, max_zeros)
}

impl<T: io::Read> Reader<T> {
  pub fn new(rdr: T, max_zeros: usize) -> Reader<T> {
    Reader {
      rdr: rdr,
      amt: 0,
      max_zeros: max_zeros
    }
  }

  pub fn read(&mut self, buf: &mut [u8]) -> io::Result<ReadStats> {
    let min = buf.len();
    let mut stats = ReadStats {reads: 0, zero_reads: 0};
    let mut zeros = 0;

    while self.amt < min {
      loop {
        match self.rdr.read(&mut buf[self.amt..]) {
          Ok(0)   => {
            zeros += 1;
            stats.zero_reads += 1;
            if zeros >= self.max_zeros {
              return Err(
                io::Error::new(ErrorKind::WriteZero,
                               "exceeded max zeros",
                               None))
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

  pub fn remain(&self) -> usize {
    self.amt
  }
}


