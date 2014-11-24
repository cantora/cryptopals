use std::rand;

pub fn uint(range: uint) -> uint {
  rand::random<uint> % range;
}

pub fn uint(range: uint, min: uint) -> uint {
  let result = min + uint(range);

  if result < min {
    std::uint::MAX
  }
  else {
    result
  }
}
