use std::{fs, io};

use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use thiserror::Error;

use crate::ast::Expression;
use crate::eval::{EvalError, Evaluator};
use crate::read::ReadError;

mod env;

pub mod ast;
pub mod eval;
pub mod read;

pub fn run() -> Result<(), RunError> {
  let args: Vec<String> = std::env::args().collect();

  if let Some(path) = args.get(1) {
    run_file(path)
  } else {
    run_repl()
  }
}

fn run_file(path: &str) -> Result<(), RunError> {
  let source = fs::read_to_string(path)?;

  let expr = read::read(source.chars())?;
  eval::eval(expr)?;

  Ok(())
}

fn run_repl() -> Result<(), RunError> {
  println!("Zuko v1.0.0");

  let mut editor = Editor::<()>::new();
  editor.set_auto_add_history(true);

  let mut evaluator = Evaluator::new();

  loop {
    match editor.readline("> ") {
      Ok(line) => match read_and_eval_line(&mut evaluator, &line) {
        Ok(expr) => println!("{:?}", expr),
        Err(error) => println!("error: {}", error),
      },
      Err(ReadlineError::Interrupted) => break,
      Err(ReadlineError::Eof) => break,
      Err(_) => println!("error: failed to read line"),
    }
  }
  println!("Bye!");

  Ok(())
}

fn read_and_eval_line(
  evaluator: &mut Evaluator,
  line: &str,
) -> Result<Expression, RunError> {
  let expr = read::read(line.chars())?;
  let expr = evaluator.expression(expr)?;
  Ok(expr)
}

#[derive(Debug, Error)]
pub enum RunError {
  #[error("{0}")]
  Io(#[from] io::Error),
  #[error("{0}")]
  Read(#[from] ReadError),
  #[error("{0}")]
  Eval(#[from] EvalError),
}
