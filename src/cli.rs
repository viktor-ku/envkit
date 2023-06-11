use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "envkit")]
#[command(bin_name = "envkit")]
#[command(about = "env files management made simple")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Diff {
        file_a: std::path::PathBuf,

        file_b: std::path::PathBuf,

        #[arg(short, help = "do not pretty print")]
        silent: bool,
    },
}
