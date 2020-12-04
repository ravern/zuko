use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast::{self, Expr, NativeError, Symbol, SYMBOL_TRUE};

#[derive(Clone, Debug)]
pub struct Frame {
  inner: Rc<RefCell<FrameInner>>,
}

#[derive(Debug)]
struct FrameInner {
  parent: Option<Frame>,
  variables: BTreeMap<Symbol, Expr>,
}

impl Frame {
  pub fn new() -> Frame {
    Frame {
      inner: Rc::new(RefCell::new(FrameInner {
        parent: None,
        variables: BTreeMap::new(),
      })),
    }
  }

  pub fn base() -> Frame {
    use ast::Atom::*;
    use Expr::*;

    let mut frame = Frame::new();

    frame.set(SYMBOL_TRUE.clone(), Atom(Symbol(SYMBOL_TRUE.clone())));
    frame.set(
      ast::Symbol::new("tail"),
      Atom(Native(Rc::new(|arguments| {
        let list = match arguments.get(0) {
          Some(Expr::List(list)) => list,
          _ => return Err(NativeError {}),
        };

        let tail = match list {
          ast::List::Cons(node) => node.tail.clone(),
          ast::List::Nil => return Err(NativeError {}),
        };

        Ok(Expr::List(tail))
      }))),
    );
    frame.set(
      ast::Symbol::new("head"),
      Atom(Native(Rc::new(|arguments| {
        let list = match arguments.get(0) {
          Some(Expr::List(list)) => list,
          _ => return Err(NativeError {}),
        };

        let head = match list {
          ast::List::Cons(node) => node.head.clone(),
          ast::List::Nil => return Err(NativeError {}),
        };

        Ok(head)
      }))),
    );

    frame
  }

  pub fn with_parent(parent: Frame) -> Frame {
    Frame {
      inner: Rc::new(RefCell::new(FrameInner {
        parent: Some(parent),
        variables: BTreeMap::new(),
      })),
    }
  }

  pub fn get(&self, symbol: &Symbol) -> Option<Expr> {
    if let Some(expr) = self.inner.borrow().variables.get(symbol) {
      Some(expr.clone())
    } else {
      self
        .inner
        .borrow()
        .parent
        .as_ref()
        .and_then(|parent| parent.get(symbol))
    }
  }

  pub fn set(&mut self, symbol: Symbol, expr: Expr) {
    self.inner.borrow_mut().variables.insert(symbol, expr);
  }
}
