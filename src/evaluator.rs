use thiserror::Error;

use crate::ast::{self, Expr, List};

pub fn eval(expr: Expr) -> Result<Expr, EvalError> {
  use Expr::*;

  match expr {
    List(list) => eval_list(*list),
    expr => Ok(expr),
  }
}

#[derive(Debug, Error)]
pub enum EvalError {
  #[error("type is invalid")]
  InvalidType,
  #[error("expression not callable")]
  NotCallable,
}

fn eval_list(list: List) -> Result<Expr, EvalError> {
  use EvalError::*;
  use Expr::*;

  let ast::List { head, tail } = list;

  match head {
    Add => eval_add(tail),
    _ => Err(NotCallable),
  }
}

fn eval_add(tail: Vec<Expr>) -> Result<Expr, EvalError> {
  use EvalError::*;
  use Expr::*;

  let arguments = tail
    .into_iter()
    .map(|expr| match expr {
      Number(number) => Ok(number),
      _ => Err(InvalidType),
    })
    .collect::<Result<Vec<f64>, EvalError>>()?;

  Ok(Expr::Number(arguments.into_iter().sum()))
}
