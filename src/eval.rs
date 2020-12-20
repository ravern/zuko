use std::error::Error;

use thiserror::Error;

use crate::ast::{
  self, Atom, Expression, Function, List, Macro, Native, Special, Symbol,
};
use crate::env::Frame;
use crate::read;

pub fn eval(expr: Expression) -> Result<Expression, EvalError> {
  let mut evalutor = Evaluator::new();
  evalutor.eval_expr(expr)
}

pub struct Evaluator {
  frame: Frame,
}

impl Evaluator {
  pub fn new() -> Evaluator {
    let mut evaluator = Evaluator {
      frame: Frame::base(),
    };

    // Inject standard library.
    let expr = read::read(include_str!("lib.zuko").chars()).unwrap();
    evaluator.eval_expr(expr).unwrap();

    evaluator
  }

  pub fn eval_expr(
    &mut self,
    expression: Expression,
  ) -> Result<Expression, EvalError> {
    Err(EvalError::WrongArity)
  }
}

#[derive(Debug, Error)]
pub enum EvalError {
  #[error("type is invalid")]
  InvalidType,
  #[error("arity is wrong")]
  WrongArity,
  #[error("expression not callable")]
  NotCallable,
  #[error("{0}")]
  Native(Box<dyn Error>),
}
