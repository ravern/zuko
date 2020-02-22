use std::collections::BTreeMap;
use std::rc::Rc;

use thiserror::Error;

use crate::ast::{self, Expr, List};

pub fn eval(expr: Expr) -> Result<Expr, EvalError> {
  let mut evalutor = Evaluator::new();
  evalutor.eval_expr(expr)
}

pub struct Evaluator {
  scopes: Vec<Scope>,
}

impl Evaluator {
  pub fn new() -> Evaluator {
    Evaluator {
      scopes: vec![Scope::new()],
    }
  }

  pub fn eval_expr(&mut self, expr: Expr) -> Result<Expr, EvalError> {
    use Expr::*;

    match expr {
      List(list) => self.eval_list(list),
      Ident(ident) => self.eval_ident(ident),
      expr => Ok(expr),
    }
  }

  pub fn eval_list(&mut self, list: Rc<List>) -> Result<Expr, EvalError> {
    use EvalError::*;
    use Expr::*;

    let ast::List { head, tail } = list.as_ref();

    let head = head.clone();
    let tail = tail.clone();

    match head {
      Add => self.eval_call_add(tail),
      Ident(ident) => self.eval_call_ident(ident, tail),
      _ => Err(NotCallable),
    }
  }

  pub fn eval_call_add(&mut self, tail: Vec<Expr>) -> Result<Expr, EvalError> {
    use Expr::*;

    let tail = tail
      .into_iter()
      .map(|expr| {
        let expr = self.eval_expr(expr)?;
        self.as_number(expr)
      })
      .collect::<Result<Vec<f64>, EvalError>>()?;

    let result = tail.into_iter().sum();

    Ok(Number(result))
  }

  pub fn eval_call_ident(
    &mut self,
    ident: String,
    tail: Vec<Expr>,
  ) -> Result<Expr, EvalError> {
    use EvalError::*;

    match ident.as_str() {
      "define" => self.eval_call_define(tail),
      "scope" => self.eval_call_scope(tail),
      _ => Err(NotCallable),
    }
  }

  pub fn eval_call_scope(
    &mut self,
    tail: Vec<Expr>,
  ) -> Result<Expr, EvalError> {
    use EvalError::*;

    self.scopes.push(Scope::new());

    if tail.is_empty() {
      return Err(WrongArity);
    }

    let mut tail = tail
      .into_iter()
      .map(|expr| self.eval_expr(expr))
      .collect::<Result<Vec<Expr>, EvalError>>()?;

    self.scopes.pop();

    Ok(tail.pop().unwrap())
  }

  pub fn eval_call_define(
    &mut self,
    mut tail: Vec<Expr>,
  ) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 2 {
      return Err(WrongArity);
    }

    let expr = self.eval_expr(tail.pop().unwrap())?;
    let ident = self.as_ident(tail.pop().unwrap())?;

    self
      .scopes
      .last_mut()
      .unwrap()
      .variables
      .insert(ident, expr.clone());

    Ok(expr)
  }

  pub fn eval_ident(&mut self, ident: String) -> Result<Expr, EvalError> {
    use EvalError::*;

    for scope in self.scopes.iter() {
      if let Some(expr) = scope.variables.get(&ident) {
        return Ok(expr.clone());
      }
    }

    Err(UndefinedIdent(ident))
  }

  fn as_ident(&mut self, expr: Expr) -> Result<String, EvalError> {
    use EvalError::*;
    use Expr::*;

    match expr {
      Ident(ident) => Ok(ident),
      _ => Err(InvalidType),
    }
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

#[derive(Debug)]
pub struct Scope {
  variables: BTreeMap<String, Expr>,
}

impl Scope {
  fn new() -> Scope {
    Scope {
      variables: BTreeMap::new(),
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
  UndefinedIdent(String),
  #[error("expression not callable")]
  NotCallable,
}
