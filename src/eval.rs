use std::error::Error;

use thiserror::Error;

use crate::ast::{
  self, Atom, Expression, Function, List, Macro, Native, Special, Symbol,
};
use crate::env::Frame;
use crate::read;

pub fn eval(expr: Expression) -> Result<Expression, EvalError> {
  let mut evalutor = Evaluator::new();
  evalutor.expression(expr)
}

pub struct Evaluator {
  frame: Frame,
}

impl Evaluator {
  pub fn new() -> Evaluator {
    let mut evaluator = Evaluator {
      frame: Frame::new(),
    };

    // Inject standard library.
    // let expression = read::read(include_str!("lib.zuko").chars()).unwrap();
    // evaluator.expression(expression).unwrap();

    evaluator
  }

  pub fn expression(
    &mut self,
    expression: Expression,
  ) -> Result<Expression, EvalError> {
    match expression {
      Expression::List(list) => self.list(list),
      Expression::Atom(atom) => self.atom(atom),
    }
  }

  pub fn list(&mut self, list: List) -> Result<Expression, EvalError> {
    Err(EvalError::InvalidType)
  }

  pub fn atom(&mut self, atom: Atom) -> Result<Expression, EvalError> {
    match atom {
      Atom::Symbol(symbol) => self
        .frame
        .get(&symbol)
        .ok_or(EvalError::UndefinedSymbol(symbol)),
      atom => Ok(Expression::Atom(atom)),
    }
  }
}

#[derive(Debug, Error)]
pub enum EvalError {
  #[error("type is invalid")]
  InvalidType,
  #[error("arity is wrong")]
  WrongArity,
  #[error("'{0}' is undefined")]
  UndefinedSymbol(Symbol),
  #[error("expression not callable")]
  NotCallable,
  #[error("{0}")]
  Native(Box<dyn Error>),
}
