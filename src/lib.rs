use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use thiserror::Error;

use crate::ast::Expr;
use crate::eval::{EvalError, Evaluator};
use crate::read::ReadError;

mod ast;
mod eval;
mod read;

pub fn run() -> Result<(), RunError> {
  println!("Yu v0.1.0");

  let mut editor = Editor::<()>::new();
  editor.set_auto_add_history(true);

  let mut evaluator = Evaluator::new();

  loop {
    match editor.readline("> ") {
      Ok(line) => match run_line(&mut evaluator, &line) {
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

fn run_line(evaluator: &mut Evaluator, line: &str) -> Result<Expr, RunError> {
  let expr = read::read(line)?;
  let expr = evaluator.eval_expr(expr)?;
  Ok(expr)
}

#[derive(Debug, Error)]
pub enum RunError {
  #[error("{0}")]
  Read(#[from] ReadError),
  #[error("{0}")]
  Eval(#[from] EvalError),
}
