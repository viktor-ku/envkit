use super::Line;
use std::fmt::Debug;

pub struct File {
    pub name: String,
    pub path: String,
    pub errors: Vec<String>,
    pub body: Vec<Line>,
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();

        for (index, line) in self.body.iter().enumerate() {
            s.push_str(&format!("    [{}] => {:?}\n", index, line));
        }

        f.write_str(&s)
    }
}
