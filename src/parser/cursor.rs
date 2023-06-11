use super::{
    env::{Clear, Comment, Indent, IsEmpty, Line, Margin, Variable},
    k::*,
    state::State,
};

pub struct Cursor {
    pub body: Vec<Line>,
    pub variable: Variable,
    pub comment: Comment,
    pub current_line: u32,
    pub state: State,
    pub indent: Indent,
    pub margin: Margin,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            body: Vec::with_capacity(64),
            variable: Variable::new(),
            indent: Indent::new(),
            comment: Comment::new(),
            current_line: 1,
            state: Default::default(),
            margin: Margin::new(),
        }
    }

    pub fn push_margin_to_value(&mut self) {
        let v = vec![K_SPACE; self.margin.size];
        self.variable.value.push_str(&String::from_utf8(v).unwrap());
        self.margin.clear();
    }

    pub fn commit(&mut self) {
        let line = Line {
            line: self.current_line,
            indent: if self.indent.is_empty() {
                None
            } else {
                Some(self.indent.clone())
            },
            comment: if self.comment.is_empty() {
                None
            } else {
                Some(self.comment.clone())
            },
            variable: if self.variable.is_empty() {
                None
            } else {
                Some(self.variable.clone())
            },
            margin: if self.margin.is_empty() {
                None
            } else {
                Some(self.margin.clone())
            },
        };

        self.body.push(line);

        self.current_line += 1;
        self.state = Default::default();

        self.comment.clear();
        self.variable.clear();
        self.margin.clear();
        self.indent.clear();
    }
}
