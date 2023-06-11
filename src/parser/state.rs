#[derive(Debug, PartialEq)]
pub enum State {
    None,
    Indent,
    Key,
    Value,
    Margin,
    Comment,
    Quote,
}

impl Default for State {
    fn default() -> Self {
        Self::None
    }
}
