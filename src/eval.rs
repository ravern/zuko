use std::error::Error;

use thiserror::Error;

use crate::ast::{
  self, Atom, Expr, Function, List, Macro, Native, Operator, Special, Symbol,
  SYMBOL_TRUE,
};
use crate::env::Frame;
use crate::read;

pub fn eval(expr: Expr) -> Result<Expr, EvalError> {
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
    let expr = read::read(include_str!("lib.zuko")).unwrap();
    evaluator.eval_expr(expr).unwrap();

    evaluator
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
    use EvalError::{NotCallable, WrongArity};
    use List::*;

    let node = match &list {
      Cons(node) => node.as_ref(),
      Nil => return Ok(Expr::List(Nil)),
    };

    let head = node.head.clone();
    let tail = node.tail.clone();

    let head = self.eval_expr(head)?;

    let function = match head {
      Expr::Atom(Function(function)) => function,
      Expr::Atom(Macro(macr)) => return self.eval_call_macro(macr, tail),
      Expr::Atom(Native(native)) => return self.eval_call_native(native, tail),
      Expr::Atom(Special(special)) => {
        return self.eval_call_special(special, tail)
      }
      _ => return Err(NotCallable),
    };

    if tail.len() != function.parameters().len() {
      return Err(WrongArity);
    }

    let arguments = tail
      .into_iter()
      .map(|expr| self.eval_expr(expr))
      .collect::<Result<Vec<Expr>, EvalError>>()?;

    let original_frame = self.frame.clone();
    self.frame = Frame::with_parent(function.frame().clone());

    let arguments: Vec<(&ast::Symbol, Expr)> = function
      .parameters()
      .into_iter()
      .zip(arguments.into_iter())
      .collect();

    for (name, argument) in arguments {
      self.frame.set(name.clone(), argument);
    }

    let expr = self.eval_expr(function.body().clone())?;

    self.frame = original_frame;

    Ok(expr)
  }

  pub fn eval_call_macro(
    &mut self,
    macr: Macro,
    tail: List,
  ) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 1 {
      return Err(WrongArity);
    }

    let original_frame = self.frame.clone();
    self.frame = Frame::new();

    let argument = tail.get(0).unwrap().clone();
    self.frame.set(macr.parameter().clone(), argument);

    let mut expr = self.eval_expr(macr.body().clone())?;

    self.frame = original_frame;

    expr = self.eval_expr(expr)?;

    Ok(expr)
  }

  pub fn eval_call_special(
    &mut self,
    special: Special,
    tail: List,
  ) -> Result<Expr, EvalError> {
    use Special::*;

    match special {
      Begin => self.eval_call_special_begin(tail),
      Define => self.eval_call_special_define(tail),
      Function => self.eval_call_special_function(tail),
      Macro => self.eval_call_special_macro(tail),
      If => self.eval_call_special_if(tail),
      Quote => self.eval_call_special_quote(tail),
      Operator(operator) => self.eval_call_special_operator(operator, tail),
    }
  }

  pub fn eval_call_native(
    &mut self,
    native: Native,
    tail: List,
  ) -> Result<Expr, EvalError> {
    let arguments = tail
      .into_iter()
      .map(|expr| self.eval_expr(expr))
      .collect::<Result<Vec<Expr>, EvalError>>()?;

    native.call(arguments)
  }

  pub fn eval_call_special_begin(
    &mut self,
    tail: List,
  ) -> Result<Expr, EvalError> {
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

  pub fn eval_call_special_define(
    &mut self,
    tail: List,
  ) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 2 {
      return Err(WrongArity);
    }

    let symbol = self.as_symbol(tail.get(0).unwrap().clone())?;
    let expr = self.eval_expr(tail.get(1).unwrap().clone())?;

    self.frame.set(symbol, expr.clone());

    Ok(expr)
  }

  pub fn eval_call_special_function(
    &mut self,
    tail: List,
  ) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 2 {
      return Err(WrongArity);
    }

    let parameters = self.as_list(tail.get(0).unwrap().clone())?;
    let body = tail.get(1).unwrap().clone();

    let parameters = parameters
      .into_iter()
      .map(|expr| self.as_symbol(expr))
      .collect::<Result<Vec<Symbol>, EvalError>>()?;

    let frame = self.frame.clone();

    Ok(Expr::Atom(Atom::Function(Function::new(
      frame, parameters, body,
    ))))
  }

  pub fn eval_call_special_macro(
    &mut self,
    tail: List,
  ) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 2 {
      return Err(WrongArity);
    }

    let parameters = self.as_list(tail.get(0).unwrap().clone())?;
    let body = tail.get(1).unwrap().clone();

    if parameters.len() != 1 {
      return Err(WrongArity);
    }

    let parameter = self.as_symbol(parameters.get(0).unwrap().clone())?;

    Ok(Expr::Atom(Atom::Macro(Macro::new(parameter, body))))
  }

  pub fn eval_call_special_if(
    &mut self,
    tail: List,
  ) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 3 {
      return Err(WrongArity);
    }

    let condition = self.eval_expr(tail.get(0).unwrap().clone())?;

    if condition.is_truthy() {
      self.eval_expr(tail.get(1).unwrap().clone())
    } else {
      self.eval_expr(tail.get(2).unwrap().clone())
    }
  }

  pub fn eval_call_special_quote(
    &mut self,
    tail: List,
  ) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 1 {
      return Err(WrongArity);
    }

    let expr = tail.get(0).unwrap().clone();

    Ok(expr)
  }

  pub fn eval_call_special_operator(
    &mut self,
    operator: Operator,
    tail: List,
  ) -> Result<Expr, EvalError> {
    use ast::Atom::*;
    use ast::List::*;
    use EvalError::*;
    use Expr::*;
    use Operator::*;

    if tail.len() != 2 {
      return Err(WrongArity);
    }

    let left = tail.get(0).unwrap().clone();
    let right = tail.get(1).unwrap().clone();

    let left = self.eval_expr(left)?;
    let right = self.eval_expr(right)?;

    let result = match operator {
      Add => {
        let left = self.as_number(left)?;
        let right = self.as_number(right)?;
        Atom(Number(left + right))
      }
      Sub => {
        let left = self.as_number(left)?;
        let right = self.as_number(right)?;
        Atom(Number(left - right))
      }
      Mul => {
        let left = self.as_number(left)?;
        let right = self.as_number(right)?;
        Atom(Number(left * right))
      }
      Div => {
        let left = self.as_number(left)?;
        let right = self.as_number(right)?;
        Atom(Number(left / right))
      }
      Mod => {
        let left = self.as_number(left)?;
        let right = self.as_number(right)?;
        Atom(Number(left % right))
      }
      Eq => {
        if left == right {
          Atom(Symbol(SYMBOL_TRUE.clone()))
        } else {
          List(Nil)
        }
      }
    };

    Ok(result)
  }

  pub fn eval_atom(&mut self, atom: Atom) -> Result<Expr, EvalError> {
    use Atom::*;

    match atom {
      Symbol(symbol) => self.eval_symbol(symbol),
      atom => Ok(Expr::Atom(atom)),
    }
  }

  pub fn eval_symbol(&mut self, symbol: Symbol) -> Result<Expr, EvalError> {
    use EvalError::*;

    match self.frame.get(&symbol) {
      Some(expr) => Ok(expr.clone()),
      None => Err(UndefinedSymbol(symbol)),
    }
  }

  fn as_symbol(&mut self, expr: Expr) -> Result<Symbol, EvalError> {
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
  UndefinedSymbol(Symbol),
  #[error("expression not callable")]
  NotCallable,
  #[error("{0}")]
  Native(Box<dyn Error>),
}
