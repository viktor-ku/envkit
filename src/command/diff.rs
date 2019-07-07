use crate::parser::env::{File, Line};
use std::collections::HashMap;

pub fn diff(a: &File, b: &File) {
  let mut a_map: HashMap<String, Line> = HashMap::with_capacity(64);

  for line in &a.body {
    if let Some(var) = &line.variable {
      a_map.insert(var.key.clone(), line.clone());
    }
  }

  for line in &b.body {
    if let Some(var) = &line.variable {
      a_map.remove(&var.key);
    }
  }

  println!("Next variables were found in file_a, but not in file_b:");
  for (key, var) in a_map.iter() {
    println!("  {} (line {})", key, var.line);
  }
}
