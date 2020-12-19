use crate::ast::{self, Atom, Expression, List, Symbol};
use crate::env::Frame;
use crate::eval::EvalError;

pub fn build_base_frame() -> Frame {
  use ast::Atom::*;
  use ast::Native;
  use ast::Symbol;
  use Expression::*;

  let mut frame = Frame::new();

  frame.set(Symbol::new("true"), Atom(Symbol(Symbol::new("true"))));

  frame.set(Symbol::new("print"), Atom(Native(Native::new(print))));
  frame.set(Symbol::new("car"), Atom(Native(Native::new(car))));
  frame.set(Symbol::new("cdr"), Atom(Native(Native::new(cdr))));
  frame.set(Symbol::new("cons"), Atom(Native(Native::new(cons))));

  frame.set(Symbol::new("number?"), Atom(Native(Native::new(is_number))));
  frame.set(Symbol::new("string?"), Atom(Native(Native::new(is_string))));
  frame.set(Symbol::new("symbol?"), Atom(Native(Native::new(is_symbol))));
  frame.set(
    Symbol::new("function?"),
    Atom(Native(Native::new(is_function))),
  );
  frame.set(
    Symbol::new("special?"),
    Atom(Native(Native::new(is_special))),
  );
  frame.set(Symbol::new("native?"), Atom(Native(Native::new(is_native))));

  frame.set(Symbol::new("sqrt"), Atom(Native(Native::new(sqrt))));

  frame
}

pub fn print(arguments: Vec<Expression>) -> Result<Expression, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  Ok(expr)
}

fn car(arguments: Vec<Expression>) -> Result<Expression, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let list = match arguments.get(0) {
    Some(Expression::List(list)) => list,
    _ => return Err(InvalidType),
  };

  let car = match list {
    ast::List::Cons(node) => (*node.car).clone(),
    ast::List::Nil => return Err(InvalidType),
  };

  Ok(car)
}

fn cdr(arguments: Vec<Expression>) -> Result<Expression, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let list = match arguments.get(0) {
    Some(Expression::List(list)) => list,
    _ => return Err(InvalidType),
  };

  let cdr = match list {
    ast::List::Cons(node) => (*node.cdr).clone(),
    ast::List::Nil => return Err(InvalidType),
  };

  Ok(Expression::List(cdr))
}

fn cons(arguments: Vec<Expression>) -> Result<Expression, EvalError> {
  use EvalError::*;

  if arguments.len() != 2 {
    return Err(WrongArity);
  }

  let car = arguments.get(0).unwrap().clone();

  let cdr = match arguments.get(1) {
    Some(Expression::List(list)) => list.clone(),
    _ => return Err(InvalidType),
  };

  Ok(Expression::List(List::cons(car, cdr)))
}

pub fn is_number(arguments: Vec<Expression>) -> Result<Expression, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expression::Atom(Atom::Float(_)) = expr {
    Ok(Expression::Atom(Atom::Symbol(Symbol::new("true"))))
  } else {
    Ok(Expression::List(List::Nil))
  }
}

pub fn is_string(arguments: Vec<Expression>) -> Result<Expression, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expression::Atom(Atom::String(_)) = expr {
    Ok(Expression::Atom(Atom::Symbol(Symbol::new("true"))))
  } else {
    Ok(Expression::List(List::Nil))
  }
}

pub fn is_symbol(arguments: Vec<Expression>) -> Result<Expression, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expression::Atom(Atom::Symbol(_)) = expr {
    Ok(Expression::Atom(Atom::Symbol(Symbol::new("true"))))
  } else {
    Ok(Expression::List(List::Nil))
  }
}

pub fn is_function(
  arguments: Vec<Expression>,
) -> Result<Expression, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expression::Atom(Atom::Function(_)) = expr {
    Ok(Expression::Atom(Atom::Symbol(Symbol::new("true"))))
  } else {
    Ok(Expression::List(List::Nil))
  }
}

pub fn is_special(arguments: Vec<Expression>) -> Result<Expression, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expression::Atom(Atom::Special(_)) = expr {
    Ok(Expression::Atom(Atom::Symbol(Symbol::new("true"))))
  } else {
    Ok(Expression::List(List::Nil))
  }
}

pub fn is_native(arguments: Vec<Expression>) -> Result<Expression, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expression::Atom(Atom::Native(_)) = expr {
    Ok(Expression::Atom(Atom::Symbol(Symbol::new("true"))))
  } else {
    Ok(Expression::List(List::Nil))
  }
}

pub fn sqrt(arguments: Vec<Expression>) -> Result<Expression, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let number = match arguments.get(0) {
    Some(Expression::Atom(Atom::Float(number))) => number,
    _ => return Err(InvalidType),
  };

  Ok(Expression::Atom(Atom::Float(number.sqrt())))
}
