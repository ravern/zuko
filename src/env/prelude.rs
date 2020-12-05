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

  frame.set(Symbol::new("debug"), Atom(Native(Native::new(debug))));
  frame.set(Symbol::new("head"), Atom(Native(Native::new(head))));
  frame.set(Symbol::new("tail"), Atom(Native(Native::new(tail))));

  frame
}

pub fn debug(arguments: Vec<Expr>) -> Result<Expr, EvalError> {
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
