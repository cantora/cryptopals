
#[deriving(PartialEq, Eq, Show)]
pub struct PQCell<T1, T2> {
  pub priority: T1,
  pub value: T2
}

/*
impl<T1: Ord, T2: Eq> Ord for PQCell<T1, T2> {
  fn cmp(&self, other: &PQCell<T1, T2>) -> Ordering {
    self.priority.cmp(&other.priority)
  }
}
*/

impl<T1: PartialOrd, T2: PartialEq> PartialOrd for PQCell<T1, T2> {
  fn lt(&self, other: &PQCell<T1, T2>) -> bool {
    self.priority.lt(&other.priority)
  }
}
