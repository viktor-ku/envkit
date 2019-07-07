use structopt::StructOpt;

mod command;

mod parser;
use parser::{parse};

mod cli;
use cli::{CLI};

fn main() {
  match CLI::from_args() {
    CLI::Diff { file_a, file_b } => {
      let env_a_file = parse(&file_a);
      let env_b_file = parse(&file_b);

      command::diff(&env_a_file, &env_b_file);
    }
  }
}
