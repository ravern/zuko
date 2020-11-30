use std::{fs, io};

use thiserror::Error;

use crate::ast::{
  self, Atom, Expr, Function, List, Native, Operator, Symbol, SYMBOL_TRUE,
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
    Evaluator {
      frame: Frame::new(),
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
      Expr::Atom(Native(native)) => return self.eval_call_native(native, tail),
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

  pub fn eval_call_native(
    &mut self,
    native: Native,
    tail: List,
  ) -> Result<Expr, EvalError> {
    use Native::*;

    match native {
      Begin => self.eval_call_begin(tail),
      Debug => self.eval_call_debug(tail),
      Define => self.eval_call_define(tail),
      Function => self.eval_call_function(tail),
      Import => self.eval_call_import(tail),
      If => self.eval_call_if(tail),
      Quote => self.eval_call_quote(tail),
      Operator(operator) => self.eval_call_operator(operator, tail),
    }
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

  pub fn eval_call_debug(&mut self, tail: List) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 1 {
      return Err(WrongArity);
    }

    let expr = self.eval_expr(tail.get(0).unwrap().clone())?;

    println!("{:?}", expr);

    Ok(expr)
  }

  pub fn eval_call_define(&mut self, tail: List) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 2 {
      return Err(WrongArity);
    }

    let symbol = self.as_symbol(tail.get(0).unwrap().clone())?;
    let expr = self.eval_expr(tail.get(1).unwrap().clone())?;

    self.frame.set(symbol, expr.clone());

    Ok(expr)
  }

  pub fn eval_call_function(&mut self, tail: List) -> Result<Expr, EvalError> {
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

  pub fn eval_call_import(&mut self, tail: List) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 1 {
      return Err(WrongArity);
    }

    let path = self.as_string(tail.get(0).unwrap().clone())?;

    let source = fs::read_to_string(path.as_str())?;

    let expr = read::read(&source)?;
    self.eval_expr(expr)
  }

  pub fn eval_call_if(&mut self, tail: List) -> Result<Expr, EvalError> {
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

  pub fn eval_call_quote(&mut self, tail: List) -> Result<Expr, EvalError> {
    use EvalError::*;

    if tail.len() != 1 {
      return Err(WrongArity);
    }

    let expr = tail.get(0).unwrap().clone();

    Ok(expr)
  }

  pub fn eval_call_operator(
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

    let left = self.as_number(left)?;
    let right = self.as_number(right)?;

    let result = match operator {
      Add => Atom(Number(left + right)),
      Sub => Atom(Number(left - right)),
      Mul => Atom(Number(left * right)),
      Div => Atom(Number(left / right)),
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

    if let Some(expr) = self.eval_native_symbol(&symbol) {
      return Ok(expr);
    }

    match self.frame.get(&symbol) {
      Some(expr) => Ok(expr.clone()),
      None => Err(UndefinedSymbol(symbol)),
    }
  }

  pub fn eval_native_symbol(&mut self, symbol: &Symbol) -> Option<Expr> {
    use ast::Operator::*;
    use Native::*;

    let native = match symbol.as_str() {
      "begin" => Begin,
      "debug" => Debug,
      "define" => Define,
      "function" => Function,
      "import" => Import,
      "if" => If,
      "quote" => Quote,
      "true" => return Some(Expr::Atom(Atom::Symbol(SYMBOL_TRUE.clone()))),
      "+" => Operator(Add),
      "-" => Operator(Sub),
      "*" => Operator(Mul),
      "/" => Operator(Div),
      "=" => Operator(Eq),
      _ => return None,
    };

    Some(Expr::Atom(Atom::Native(native)))
  }

  fn as_symbol(&mut self, expr: Expr) -> Result<Symbol, EvalError> {
    use Atom::*;
    use EvalError::*;

    match expr {
      Expr::Atom(Symbol(symbol)) => Ok(symbol),
      _ => Err(InvalidType),
    }
  }

  fn as_string(&mut self, expr: Expr) -> Result<String, EvalError> {
    use Atom::*;
    use EvalError::*;

    match expr {
      Expr::Atom(String(string)) => Ok(string),
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
  #[error("{0}")]
  Io(#[from] io::Error),
  #[error("{0}")]
  Read(#[from] read::ReadError),
  #[error("type is invalid")]
  InvalidType,
  #[error("arity is wrong")]
  WrongArity,
  #[error("'{0}' is undefined")]
  UndefinedSymbol(Symbol),
  #[error("expression not callable")]
  NotCallable,
}
