use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast::{Expr, Symbol};

pub use self::prelude::BASE_FRAME;

mod prelude;

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
