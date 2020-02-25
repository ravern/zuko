use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast::Expr;

#[derive(Clone, Debug)]
pub struct Frame {
  inner: Rc<RefCell<FrameInner>>,
}

#[derive(Debug)]
struct FrameInner {
  parent: Option<Frame>,
  variables: BTreeMap<String, Expr>,
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

  pub fn get(&self, symbol: &str) -> Option<Expr> {
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

  pub fn set(&mut self, symbol: String, expr: Expr) {
    self.inner.borrow_mut().variables.insert(symbol, expr);
  }
}
