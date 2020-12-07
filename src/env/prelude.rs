use crate::ast::{self, Atom, Expr, List, SYMBOL_TRUE};
use crate::env::Frame;
use crate::eval::EvalError;

pub fn build_base_frame() -> Frame {
  use ast::Atom::*;
  use ast::Native;
  use ast::Symbol;
  use Expr::*;

  let mut frame = Frame::new();

  frame.set(SYMBOL_TRUE.clone(), Atom(Symbol(SYMBOL_TRUE.clone())));

  frame.set(Symbol::new("print"), Atom(Native(Native::new(print))));
  frame.set(Symbol::new("head"), Atom(Native(Native::new(head))));
  frame.set(Symbol::new("tail"), Atom(Native(Native::new(tail))));
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

  frame
}

pub fn print(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  println!("{}", expr);

  Ok(expr)
}

fn head(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let list = match arguments.get(0) {
    Some(Expr::List(list)) => list,
    _ => return Err(InvalidType),
  };

  let head = match list {
    ast::List::Cons(node) => node.head.clone(),
    ast::List::Nil => return Err(InvalidType),
  };

  Ok(head)
}

fn tail(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let list = match arguments.get(0) {
    Some(Expr::List(list)) => list,
    _ => return Err(InvalidType),
  };

  let tail = match list {
    ast::List::Cons(node) => node.tail.clone(),
    ast::List::Nil => return Err(InvalidType),
  };

  Ok(Expr::List(tail))
}

fn cons(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
  use EvalError::*;

  if arguments.len() != 2 {
    return Err(WrongArity);
  }

  let head = arguments.get(0).unwrap().clone();

  let tail = match arguments.get(1) {
    Some(Expr::List(list)) => list.clone(),
    _ => return Err(InvalidType),
  };

  Ok(Expr::List(List::cons(head, tail)))
}

pub fn is_number(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expr::Atom(Atom::Number(_)) = expr {
    Ok(Expr::Atom(Atom::Symbol(SYMBOL_TRUE.clone())))
  } else {
    Ok(Expr::List(List::Nil))
  }
}

pub fn is_string(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expr::Atom(Atom::String(_)) = expr {
    Ok(Expr::Atom(Atom::Symbol(SYMBOL_TRUE.clone())))
  } else {
    Ok(Expr::List(List::Nil))
  }
}

pub fn is_symbol(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expr::Atom(Atom::Symbol(_)) = expr {
    Ok(Expr::Atom(Atom::Symbol(SYMBOL_TRUE.clone())))
  } else {
    Ok(Expr::List(List::Nil))
  }
}

pub fn is_function(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expr::Atom(Atom::Function(_)) = expr {
    Ok(Expr::Atom(Atom::Symbol(SYMBOL_TRUE.clone())))
  } else {
    Ok(Expr::List(List::Nil))
  }
}

pub fn is_special(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expr::Atom(Atom::Special(_)) = expr {
    Ok(Expr::Atom(Atom::Symbol(SYMBOL_TRUE.clone())))
  } else {
    Ok(Expr::List(List::Nil))
  }
}

pub fn is_native(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
  use EvalError::*;

  if arguments.len() != 1 {
    return Err(WrongArity);
  }

  let expr = arguments.get(0).unwrap().clone();

  if let Expr::Atom(Atom::Native(_)) = expr {
    Ok(Expr::Atom(Atom::Symbol(SYMBOL_TRUE.clone())))
  } else {
    Ok(Expr::List(List::Nil))
  }
}
