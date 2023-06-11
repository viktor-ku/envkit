use super::{Clear, IsEmpty};

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    /// Includes spaces before and after the text
    pub value: String,
}

impl Comment {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }
}

impl Clear for Comment {
    fn clear(&mut self) {
        self.value.clear();
    }
}

impl IsEmpty for Comment {
    fn is_empty(&self) -> bool {
        self.value.len() == 0
    }
}
