use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
pub struct PQCell<T1, T2> {
  pub priority: T1,
  pub value: T2
}

impl<T1: PartialOrd, T2: PartialEq> PartialOrd for PQCell<T1, T2> {
  fn partial_cmp(&self, other: &PQCell<T1, T2>) -> Option<Ordering> {
    self.priority.partial_cmp(&other.priority)
  }
}
