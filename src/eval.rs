use thiserror::Error;

use crate::ast::{Atom, Expr, Function, List, Scope};

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
      Atom(atom) => self.eval_atom(atom),
    }
  }

  pub fn eval_list(&mut self, list: List) -> Result<Expr, EvalError> {
    use Atom::*;
    use EvalError::*;

    if list.is_nil() {
      return Ok(Expr::List(list));
    }

    let (head, tail) = list.decons().unwrap();

    if let Expr::Atom(Symbol(symbol)) = head.clone() {
      if let Some(expr) = self.eval_call_special(symbol, tail.clone())? {
        return Ok(expr);
      }
    }

    let head = self.eval_expr(head)?;

    let function = match head {
      Expr::Atom(Function(function)) => function,
      _ => return Err(NotCallable),
    };

    if tail.len() != function.parameters.len() {
      return Err(WrongArity);
    }

    self.scopes.push(function.scope.clone());

    function
      .parameters
      .into_iter()
      .zip(tail.into_iter())
      .map(|(symbol, expr)| {
        self.eval_call_define(List::cons(
          Expr::Atom(Symbol(symbol)),
          List::cons(expr, List::nil()),
        ))
      })
      .collect::<Result<Vec<Expr>, EvalError>>()?;

    let expr = self.eval_expr(function.body)?;

    self.scopes.pop();

    Ok(expr)
  }

  pub fn eval_call_special(
    &mut self,
    head: String,
    tail: List,
  ) -> Result<Option<Expr>, EvalError> {
    let result = match head.as_str() {
      "begin" => self.eval_call_begin(tail),
      "define" => self.eval_call_define(tail),
      "function" => self.eval_call_function(tail),
      "quote" => self.eval_call_quote(tail),
      _ => return Ok(None),
    };

    result.map(Some)
  }

  pub fn eval_call_begin(&mut self, tail: List) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() < 1 {
      return Err(WrongArity);
    }

    let mut tail = tail
      .into_iter()
      .map(|expr| self.eval_expr(expr))
      .collect::<Result<Vec<Expr>, EvalError>>()?;

    Ok(tail.pop().unwrap())
  }

  pub fn eval_call_define(&mut self, tail: List) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 2 {
      return Err(WrongArity);
    }

    let (head, tail) = tail.decons().unwrap();
    let symbol = self.as_symbol(head)?;
    let (head, _) = tail.decons().unwrap();
    let expr = self.eval_expr(head)?;

    self.scopes.last_mut().unwrap().set(symbol, expr.clone());

    Ok(expr)
  }

  pub fn eval_call_function(&mut self, tail: List) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 2 {
      return Err(WrongArity);
    }

    let (head, tail) = tail.decons().unwrap();
    let parameters = self.as_list(head)?;
    let (head, _) = tail.decons().unwrap();
    let body = head;

    let parameters = parameters
      .into_iter()
      .map(|expr| self.as_symbol(expr))
      .collect::<Result<Vec<String>, EvalError>>()?;

    let scope = self.scopes.last().unwrap().clone();

    Ok(Expr::Atom(Atom::Function(Box::new(Function {
      scope,
      parameters,
      body,
    }))))
  }

  pub fn eval_call_quote(&mut self, tail: List) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 1 {
      return Err(WrongArity);
    }

    let (head, _) = tail.decons().unwrap();
    let expr = head;

    Ok(expr)
  }

  pub fn eval_atom(&mut self, atom: Atom) -> Result<Expr, EvalError> {
    use Atom::*;

    match atom {
      Symbol(symbol) => self.eval_symbol(symbol),
      atom => Ok(Expr::Atom(atom)),
    }
  }

  pub fn eval_symbol(&mut self, symbol: String) -> Result<Expr, EvalError> {
    use EvalError::*;

    for scope in self.scopes.iter() {
      if let Some(expr) = scope.get(&symbol) {
        return Ok(expr.clone());
      }
    }

    Err(UndefinedSymbol(symbol))
  }

  fn as_symbol(&mut self, expr: Expr) -> Result<String, EvalError> {
    use Atom::*;
    use EvalError::*;

    match expr {
      Expr::Atom(Symbol(symbol)) => Ok(symbol),
      _ => Err(InvalidType),
    }
  }

  fn as_list(&mut self, expr: Expr) -> Result<List, EvalError> {
    use EvalError::*;
    use Expr::*;

    match expr {
      List(list) => Ok(list),
      _ => Err(InvalidType),
    }
  }

  fn as_number(&mut self, expr: Expr) -> Result<f64, EvalError> {
    use Atom::*;
    use EvalError::*;

    match expr {
      Expr::Atom(Number(number)) => Ok(number),
      _ => Err(InvalidType),
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
  UndefinedSymbol(String),
  #[error("expression not callable")]
  NotCallable,
}
