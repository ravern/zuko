use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast::Expr;

#[derive(Debug)]
pub struct Frame {
  parent: Option<Rc<Frame>>,
  variables: BTreeMap<String, Expr>,
}

impl Frame {
  pub fn new() -> Rc<Frame> {
    Rc::new(Frame {
      parent: None,
      variables: BTreeMap::new(),
    })
  }

  pub fn with_parent(parent: Rc<Frame>) -> Rc<Frame> {
    Rc::new(Frame {
      parent: Some(parent),
      variables: BTreeMap::new(),
    })
  }

  pub fn get(&self, symbol: &str) -> Option<Expr> {
    if let Some(expr) = self.variables.get(symbol) {
      Some(expr.clone())
    } else {
      self.parent.as_ref().and_then(|parent| parent.get(symbol))
    }
  }

  pub fn set(&self, symbol: String, expr: Expr) -> Rc<Frame> {
    let parent = self.parent.clone();

    let mut variables = self.variables.clone();
    variables.insert(symbol, expr);

    Rc::new(Frame { parent, variables })
  }
}
