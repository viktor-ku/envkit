use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "envkit", about = "the stupid .env management tool")]
pub enum CLI {
  #[structopt(name = "diff")]
  Diff {
    #[structopt(
      parse(from_os_str),
    )]
    file_a: std::path::PathBuf,

    #[structopt(
      parse(from_os_str),
    )]
    file_b: std::path::PathBuf,

    #[structopt(
      short = "s",
      long = "silent",
      help = "Prints out simple key=value pairs without formatting"
    )]
    silent: bool,
  }
}
