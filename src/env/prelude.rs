use lazy_static::lazy_static;

use crate::ast::{self, Expr, NativeError, SYMBOL_TRUE};
use crate::env::Frame;

lazy_static! {
  pub static ref BASE_FRAME: Frame = {
    use ast::Atom::*;
    use ast::Native;
    use ast::Symbol;
    use Expr::*;

    let mut frame = Frame::new();

    frame.set(SYMBOL_TRUE.clone(), Atom(Symbol(SYMBOL_TRUE.clone())));

    frame.set(Symbol::new("head"), Atom(Native(Native::new(head))));
    frame.set(Symbol::new("tail"), Atom(Native(Native::new(tail))));

    frame
  };
}

fn head(arguments: Vec<Expr>) -> Result<Expr, NativeError> {
  let list = match arguments.get(0) {
    Some(Expr::List(list)) => list,
    _ => return Err(NativeError {}),
  };

  let head = match list {
    ast::List::Cons(node) => node.head.clone(),
    ast::List::Nil => return Err(NativeError {}),
  };

  Ok(head)
}

fn tail(arguments: Vec<Expr>) -> Result<Expr, NativeError> {
  let list = match arguments.get(0) {
    Some(Expr::List(list)) => list,
    _ => return Err(NativeError {}),
  };

  let tail = match list {
    ast::List::Cons(node) => node.tail.clone(),
    ast::List::Nil => return Err(NativeError {}),
  };

  Ok(Expr::List(tail))
}
