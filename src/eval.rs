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
    Evaluator {
      frame: Frame::new(),
    }
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
    if let List::Nil = list {
      return Ok(Expression::List(list));
    }

    let callable = self.expression(list.car().unwrap().clone())?;

    match callable {
      Expression::Atom(Atom::Function(_)) => self.function(list),
      Expression::Atom(Atom::Macro(_)) => self.macro_(list),
      Expression::Atom(Atom::Special(_)) => self.special(list),
      Expression::Atom(Atom::Native(_)) => self.native(list),
      _ => Err(EvalError::NotCallable),
    }
  }

  pub fn function(&mut self, list: List) -> Result<Expression, EvalError> {
    let function = match list.car().unwrap() {
      Expression::Atom(Atom::Function(function)) => function,
      _ => return Err(EvalError::NotCallable),
    };

    let arguments = list
      .cdr()
      .unwrap()
      .clone()
      .into_iter()
      .map(|expression| self.expression(expression))
      .collect::<Result<Vec<Expression>, EvalError>>()?;

    let original_frame = self.frame.clone();
    self.frame = Frame::with_parent(self.frame.clone());

    for (parameter, argument) in
      function.parameters().into_iter().zip(arguments.into_iter())
    {
      self.frame.set(parameter.clone(), argument);
    }

    let result = self.expression(function.body().clone());

    self.frame = original_frame;

    result
  }

  pub fn macro_(&mut self, list: List) -> Result<Expression, EvalError> {
    let macro_ = match list.car().unwrap() {
      Expression::Atom(Atom::Macro(macro_)) => macro_,
      _ => return Err(EvalError::NotCallable),
    };

    let argument = list.get(1).ok_or(EvalError::WrongArity)?.clone();

    let original_frame = self.frame.clone();
    self.frame = Frame::with_parent(self.frame.clone());

    self.frame.set(macro_.parameter().clone(), argument);

    let body = self.expression(macro_.body().clone())?;

    self.frame = original_frame;

    self.expression(body)
  }

  pub fn special(&mut self, list: List) -> Result<Expression, EvalError> {
    let special = match list.car().unwrap() {
      Expression::Atom(Atom::Special(special)) => special,
      _ => return Err(EvalError::NotCallable),
    };

    let arguments = list.cdr().unwrap().clone();

    match special {
      Special::Do => self.special_do(arguments),
      Special::Define => self.special_define(arguments),
      Special::Function => self.special_function(arguments),
      Special::Macro => self.special_macro(arguments),
      Special::If => self.special_if(arguments),
      Special::Quote => Ok(list.get(1).ok_or(EvalError::WrongArity)?.clone()),
    }
  }

  pub fn special_do(
    &mut self,
    arguments: List,
  ) -> Result<Expression, EvalError> {
    arguments
      .into_iter()
      .map(|expression| self.expression(expression))
      .collect::<Result<Vec<Expression>, EvalError>>()?
      .pop()
      .ok_or(EvalError::WrongArity)
  }

  pub fn special_define(
    &mut self,
    arguments: List,
  ) -> Result<Expression, EvalError> {
    let name = match arguments.car().ok_or(EvalError::WrongArity)? {
      Expression::Atom(Atom::Symbol(symbol)) => symbol.clone(),
      _ => return Err(EvalError::InvalidType),
    };

    let expression = arguments.get(1).ok_or(EvalError::WrongArity)?.clone();

    self.frame.set(name, expression.clone());

    Ok(expression)
  }

  pub fn special_function(
    &mut self,
    arguments: List,
  ) -> Result<Expression, EvalError> {
    let parameters = match arguments.car().ok_or(EvalError::WrongArity)? {
      Expression::List(list) => list.clone(),
      _ => return Err(EvalError::InvalidType),
    };

    let parameters = parameters
      .into_iter()
      .map(|expression| match expression {
        Expression::Atom(Atom::Symbol(parameter)) => Ok(parameter),
        _ => Err(EvalError::InvalidType),
      })
      .collect::<Result<Vec<Symbol>, EvalError>>()?;

    let body = arguments.get(1).ok_or(EvalError::WrongArity)?.clone();

    Ok(Expression::Atom(Atom::Function(Function::new(
      self.frame.clone(),
      parameters,
      body,
    ))))
  }

  pub fn special_macro(
    &mut self,
    arguments: List,
  ) -> Result<Expression, EvalError> {
    let parameter = match arguments.car().ok_or(EvalError::WrongArity)? {
      Expression::Atom(Atom::Symbol(parameter)) => parameter.clone(),
      _ => return Err(EvalError::InvalidType),
    };

    let body = arguments.get(1).ok_or(EvalError::WrongArity)?.clone();

    Ok(Expression::Atom(Atom::Macro(Macro::new(parameter, body))))
  }

  pub fn special_if(
    &mut self,
    arguments: List,
  ) -> Result<Expression, EvalError> {
    let condition =
      self.expression(arguments.car().ok_or(EvalError::WrongArity)?.clone())?;

    if condition.is_truthy() {
      self.expression(arguments.get(1).ok_or(EvalError::WrongArity)?.clone())
    } else {
      self.expression(arguments.get(2).ok_or(EvalError::WrongArity)?.clone())
    }
  }

  pub fn native(&mut self, list: List) -> Result<Expression, EvalError> {
    let native = match list.car().unwrap() {
      Expression::Atom(Atom::Native(native)) => native,
      _ => return Err(EvalError::NotCallable),
    };

    let arguments = list.cdr().unwrap().clone();

    native.call(arguments)
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
