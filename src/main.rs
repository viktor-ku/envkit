use structopt::StructOpt;

mod command;

mod parser;
use parser::{parse};

mod cli;
use cli::{CLI};

fn main() {
  match CLI::from_args() {
    CLI::Diff { file_a, file_b } => {
      let canon_file_a = file_a.canonicalize().unwrap();
      let canon_file_b = file_b.canonicalize().unwrap();

      let env_a_file = parse(&canon_file_a);
      let env_b_file = parse(&canon_file_b);

      command::diff(&env_a_file, &env_b_file);
    }
  }
}
