use super::{Comment, Indent, IsEmpty, Margin, Variable};
use std::fmt::Debug;

#[derive(PartialEq, Clone)]
pub struct Line {
    pub line: u32,
    pub indent: Option<Indent>,
    pub variable: Option<Variable>,
    pub margin: Option<Margin>,
    pub comment: Option<Comment>,
}

impl IsEmpty for Line {
    fn is_empty(&self) -> bool {
        self.indent.is_none()
            && self.variable.is_none()
            && self.margin.is_none()
            && self.comment.is_none()
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::from("EnvLine {");

        s.push_str("\n");

        s.push_str(&format!("      line: {}\n", self.line));
        s.push_str(&format!("      indent: {:?}\n", self.indent));
        s.push_str(&format!("      variable: {:?}\n", self.variable));
        s.push_str(&format!("      margin: {:?}\n", self.margin));
        s.push_str(&format!("      comment: {:?}\n", self.comment));

        s.push_str("    }\n");
        f.write_str(&s)
    }
}
