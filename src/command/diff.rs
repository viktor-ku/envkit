use crate::parser::env::{File, Line};
use std::collections::HashMap;
use ansi_term::Colour;

fn silent(a: &File, b: &File, a_map: &HashMap<String, Line>) {
  for (key, line) in a_map.iter() {
    print!("{}=", &key);

    let value = &line.variable.as_ref().unwrap().value;

    if !value.is_empty() {
      print!("{}", value);
    }

    println!();
  }
}

fn standard(a: &File, b: &File, a_map: &HashMap<String, Line>) {
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
}

pub fn diff(a: &File, b: &File, is_silent: bool) {
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
    std::process::exit(0);
  }

  if is_silent {
    silent(&a, &b, &a_map);
  } else {
    standard(&a, &b, &a_map);
  }

  std::process::exit(1);
}
