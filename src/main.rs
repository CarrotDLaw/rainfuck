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
  let source_code = fs::read_to_string(&args.source_path)
    .expect(&InterpreterError::SourcePathError(args.source_path).to_string());

  Computer::new().interpreter(&source_code).unwrap();

  // tests
}
