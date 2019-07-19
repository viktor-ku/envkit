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

  for (key, line) in a_map.iter() {
    print!("{}", Colour::Red.paint(&key.to_string()));

    let value = &line.variable.as_ref().unwrap().value;

    if !value.is_empty() {
      print!(" {}", Colour::White.dimmed().paint("=> ["));
      print!("{}", value);
      print!("{}", Colour::White.dimmed().paint("]"));
    }

    println!(
      " {}",
      Colour::White.dimmed().paint(format!("found at {}:{}", &a.path, &line.line)),
    );
  }

  std::process::exit(1);
}
