use thiserror::Error;

use crate::ast::{self, Expr, List};

pub fn eval(expr: Expr) -> Result<Expr, EvalError> {
  let mut evalutor = Evaluator::new();
  evalutor.eval_expr(expr)
}

pub struct Evaluator {}

impl Evaluator {
  pub fn new() -> Evaluator {
    Evaluator {}
  }

  pub fn eval_expr(&mut self, expr: Expr) -> Result<Expr, EvalError> {
    use Expr::*;

    match expr {
      List(list) => self.eval_list(*list),
      expr => Ok(expr),
    }
  }

  pub fn eval_list(&mut self, list: List) -> Result<Expr, EvalError> {
    use EvalError::*;
    use Expr::*;

    let ast::List { head, tail } = list;

    let head = self.eval_expr(head)?;
    let tail = tail
      .into_iter()
      .map(|expr| self.eval_expr(expr))
      .collect::<Result<Vec<Expr>, EvalError>>()?;

    match head {
      Add => self.eval_add(tail),
      _ => Err(NotCallable),
    }
  }

  pub fn eval_add(&mut self, tail: Vec<Expr>) -> Result<Expr, EvalError> {
    use Expr::*;

    let tail = tail
      .into_iter()
      .map(|expr| self.as_number(expr))
      .collect::<Result<Vec<f64>, EvalError>>()?;

    let result = tail.into_iter().sum();

    Ok(Number(result))
  }

  fn as_number(&mut self, expr: Expr) -> Result<f64, EvalError> {
    use EvalError::*;
    use Expr::*;

    match expr {
      Number(number) => Ok(number),
      _ => Err(InvalidType),
    }
  }
}

#[derive(Debug, Error)]
pub enum EvalError {
  #[error("type is invalid")]
  InvalidType,
  #[error("expression not callable")]
  NotCallable,
}
