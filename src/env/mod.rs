use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast::{Expression, Symbol};

pub use self::prelude::build_base_frame;

mod prelude;

#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
  inner: Rc<RefCell<FrameInner>>,
}

#[derive(Debug, PartialEq)]
struct FrameInner {
  parent: Option<Frame>,
  variables: BTreeMap<Symbol, Expression>,
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
    build_base_frame()
  }

  pub fn with_parent(parent: Frame) -> Frame {
    Frame {
      inner: Rc::new(RefCell::new(FrameInner {
        parent: Some(parent),
        variables: BTreeMap::new(),
      })),
    }
  }

  pub fn get(&self, symbol: &Symbol) -> Option<Expression> {
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

  pub fn set(&mut self, symbol: Symbol, expr: Expression) {
    self.inner.borrow_mut().variables.insert(symbol, expr);
  }
}
