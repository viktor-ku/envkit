pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

pub trait Clear {
    fn clear(&mut self);
}

mod comment;
pub use comment::Comment;

mod file;
pub use file::File;

mod margin;
pub use margin::Margin;

mod indent;
pub use indent::Indent;

mod line;
pub use line::Line;

mod variable;
pub use variable::Variable;
