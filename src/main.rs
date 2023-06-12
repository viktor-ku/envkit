mod command;
use clap::Parser;

mod parser;
use parser::parse;

mod cli;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Diff {
            file_a,
            file_b,
            silent,
        } => {
            let canon_file_a = file_a.canonicalize().unwrap();
            let canon_file_b = file_b.canonicalize().unwrap();

            let env_a_file = parse(&canon_file_a);
            let env_b_file = parse(&canon_file_b);

            command::diff(&env_a_file, &env_b_file, silent);
        }
        Commands::Swap { file_a, file_b } => {
            let canon_file_a = file_a.canonicalize().unwrap();
            let canon_file_b = file_b.canonicalize().unwrap();

            command::swap(canon_file_a, canon_file_b).unwrap();
        }
    };
}
