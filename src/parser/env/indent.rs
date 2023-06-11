use super::{Clear, IsEmpty};

#[derive(Debug, Clone, PartialEq)]
pub struct Indent {
    pub size: usize,
}

impl Indent {
    pub fn new() -> Self {
        Self { size: 0 }
    }
}

impl Clear for Indent {
    fn clear(&mut self) {
        self.size = 0;
    }
}

impl IsEmpty for Indent {
    fn is_empty(&self) -> bool {
        self.size == 0
    }
}
