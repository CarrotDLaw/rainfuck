use clap::Parser;
use rainfuck::{Computer, InterpreterError};
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  source_path: String,
}

fn main() {
  let args = Args::parse();
  match fs::read_to_string(&args.source_path) {
    Ok(source_code) => Computer::new().interpreter(&source_code).unwrap(),
    Err(_) => eprintln!("{}", InterpreterError::SourcePathError(args.source_path)),
  }

  // tests
}
