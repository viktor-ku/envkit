use super::{Clear, IsEmpty};

#[derive(Debug, Clone, PartialEq)]
pub struct Margin {
    pub size: usize,
}

impl Margin {
    pub fn new() -> Self {
        Self { size: 0 }
    }
}

impl Clear for Margin {
    fn clear(&mut self) {
        self.size = 0;
    }
}

impl IsEmpty for Margin {
    fn is_empty(&self) -> bool {
        self.size == 0
    }
}
