use super::{Clear, IsEmpty};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
  pub key: String,
  pub value: String,
}

impl Variable {
  pub fn new() -> Self {
    Self {
      key: String::with_capacity(32),
      value: String::with_capacity(64),
    }
  }
}

impl Clear for Variable {
  fn clear(&mut self) {
    self.key.clear();
    self.value.clear();
  }
}

impl IsEmpty for Variable {
  fn is_empty(&self) -> bool {
    self.key.len() == 0 && self.value.len() == 0
  }
}
