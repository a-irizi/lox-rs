use clap::Parser;
use rox::{run_file, run_prompt};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Cli {
  /// Source file name.
  filename: Option<PathBuf>,
}

fn main() {
  let cli = Cli::parse();

  match cli.filename {
    Some(path) => run_file(path).expect("Failed to run file"),
    None => run_prompt(),
  }
}
