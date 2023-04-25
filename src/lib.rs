use std::io::{stdin, Read, Stdin};
use thiserror::Error;

const TAPE_SIZE: usize = 30_000;

#[derive(Debug, Error)]
pub enum InterpreterError {
  #[error("Error: Could not read source code from {0}")]
  SourcePathError(String),

  #[error("Error: Could not interpret source code, {0}")]
  ParseCodeError(String),

  #[error("Error: Memory overflow")]
  MemoryOverflow,

  #[error("Error: Pointer overflow")]
  PointerOverflow,

  #[error("Error: Could not read from stdin, {0}")]
  StdinError(std::io::Error),
}

#[derive(Clone)]
enum RawCode {
  PtrIncrement,
  PtrDecrement,
  Increment,
  Decrement,
  Write,
  Read,
  LoopBegin,
  LoopEnd,
}

enum OpCode {
  PtrIncrement,
  PtrDecrement,
  Increment,
  Decrement,
  Write,
  Read,
  Loop(Vec<OpCode>),
}

pub struct Computer {
  memory: Vec<u8>,
  pointer: usize,
  stdin: Stdin,
}

impl Computer {
  pub fn new() -> Self {
    Self {
      memory: vec![0; TAPE_SIZE],
      pointer: 0,
      stdin: stdin(),
    }
  }

  pub fn interpreter(&mut self, source_code: &str) -> Result<(), InterpreterError> {
    let lexed_codes = self.lexer(&source_code);
    let parsed_codes = self.parser(&lexed_codes);

    // println!("{:?}", lexed_codes);
    // println!("{:?}", parsed_codes);

    self.execute(&parsed_codes)?;
    Ok(())
  }

  fn lexer(&mut self, source_code: &str) -> Vec<RawCode> {
    source_code
      .chars()
      .filter_map(|c| match c {
        '>' => Some(RawCode::PtrIncrement),
        '<' => Some(RawCode::PtrDecrement),
        '+' => Some(RawCode::Increment),
        '-' => Some(RawCode::Decrement),
        '.' => Some(RawCode::Write),
        ',' => Some(RawCode::Read),
        '[' => Some(RawCode::LoopBegin),
        ']' => Some(RawCode::LoopEnd),
        _ => None,
      })
      .collect::<Vec<RawCode>>()
  }

  fn parser(&mut self, raw_codes: &Vec<RawCode>) -> Vec<OpCode> {
    let mut op_codes: Vec<OpCode> = Vec::new();

    let mut loop_start: usize = 0;
    let mut loop_stack: usize = 0;
    for (i, op) in raw_codes.iter().enumerate() {
      if loop_stack == 0 {
        match op {
          RawCode::PtrIncrement => op_codes.push(OpCode::PtrIncrement),
          RawCode::PtrDecrement => op_codes.push(OpCode::PtrDecrement),
          RawCode::Increment => op_codes.push(OpCode::Increment),
          RawCode::Decrement => op_codes.push(OpCode::Decrement),
          RawCode::Write => op_codes.push(OpCode::Write),
          RawCode::Read => op_codes.push(OpCode::Read),

          RawCode::LoopBegin => {
            loop_start = i;
            loop_stack += 1;
          }
          RawCode::LoopEnd => {
            eprintln!(
              "{}",
              InterpreterError::ParseCodeError(format!("loop ending at #{} has no start", i))
            )
          }
        }
      } else {
        match op {
          RawCode::LoopBegin => loop_stack += 1,
          RawCode::LoopEnd => {
            loop_stack -= 1;

            if loop_stack == 0 {
              op_codes.push(OpCode::Loop(
                self.parser(&raw_codes[loop_start + 1..i].to_vec()),
              ));
            }
          }

          _ => (),
        }
      }
    }

    if loop_stack != 0 {
      eprintln!(
        "{}",
        InterpreterError::ParseCodeError(format!("loop starting at #{} has no end", loop_start))
      );
    }

    op_codes
  }

  fn execute(&mut self, op_codes: &[OpCode]) -> Result<(), InterpreterError> {
    for op in op_codes {
      match op {
        OpCode::PtrIncrement => {
          self.pointer = self
            .pointer
            .checked_add(1)
            .ok_or(InterpreterError::PointerOverflow)?;
        }
        OpCode::PtrDecrement => {
          self.pointer = self
            .pointer
            .checked_sub(1)
            .ok_or(InterpreterError::PointerOverflow)?;
        }
        OpCode::Increment => {
          self.memory[self.pointer] = self.memory[self.pointer]
            .checked_add(1)
            .ok_or(InterpreterError::MemoryOverflow)?;
        }
        OpCode::Decrement => {
          self.memory[self.pointer] = self.memory[self.pointer]
            .checked_sub(1)
            .ok_or(InterpreterError::MemoryOverflow)?;
        }

        OpCode::Write => {
          print!("{}", self.memory[self.pointer] as char);
        }
        OpCode::Read => {
          let mut input = [0u8];
          if let Err(e) = self.stdin.read(&mut input) {
            return Err(InterpreterError::StdinError(e));
          }
          self.memory[self.pointer] = input[0];
        }

        OpCode::Loop(loop_body) => {
          while self.memory[self.pointer] != 0 {
            self.execute(&loop_body)?;
          }
        }
      }
    }

    Ok(())
  }
}
