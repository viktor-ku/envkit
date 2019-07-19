use crate::parser::env::{File, Line};
use std::collections::HashMap;
use ansi_term::Colour;

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

  if a_map.is_empty() {
    return;
  }

  println!(
    "Next variables were found in {}, but not in {}:\n",
    Colour::Green.paint(&a.name),
    Colour::Red.paint(&b.name)
  );

  for (key, var) in a_map.iter() {
    println!(
      "{} {}",
      Colour::Red.paint(format!("- {}", &key)),
      Colour::White.dimmed().paint(format!("found at {}:{}", &a.path, &var.line)),
    );
  }

  std::process::exit(1);
}
