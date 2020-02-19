use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use thiserror::Error;

use crate::evaluator::EvalError;
use crate::reader::ReadError;

mod ast;
mod evaluator;
mod reader;

pub fn run() -> Result<(), RunError> {
  println!("Yu v0.1.0");

  let mut editor = Editor::<()>::new();
  editor.set_auto_add_history(true);

  loop {
    match editor.readline("> ") {
      Ok(line) => {
        let value = reader::read(&line)?;
        let value = evaluator::eval(value)?;
        println!("{:?}", value);
      }
      Err(ReadlineError::Interrupted) => break,
      Err(ReadlineError::Eof) => break,
      Err(_) => println!("error: failed to read line"),
    }
  }
  println!("Bye!");

  Ok(())
}

#[derive(Debug, Error)]
pub enum RunError {
  #[error("reading failed: {0}")]
  Read(#[from] ReadError),
  #[error("evaluation failed: {0}")]
  Eval(#[from] EvalError),
}
