use crate::ast::{self, Expr, SYMBOL_TRUE};
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

// pub fn import(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
//   use EvalError::*;

//   if tail.len() != 1 {
//     return Err(WrongArity);
//   }

//   let path = self.as_string(tail.get(0).unwrap().clone())?;

//   let source = fs::read_to_string(path.as_str())?;

//   let expr = read::read(&source)?;
//   self.eval_expr(expr)
// }

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
  use ast::List;
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
