use std::iter::Iterator;
use std::rc::Rc;

use super::Expr;

#[derive(Clone, Debug)]
pub struct List(Option<Rc<Node>>);

#[derive(Clone, Debug)]
struct Node {
  head: Expr,
  tail: List,
}

impl List {
  pub fn cons(head: Expr, tail: List) -> List {
    let node = Node { head, tail };
    List(Some(Rc::new(node)))
  }

  pub fn nil() -> List {
    List(None)
  }

  pub fn decons(&self) -> Option<(Expr, List)> {
    if let Some(node) = self.0.as_ref() {
      let head = node.head.clone();
      let tail = node.tail.clone();
      Some((head, tail))
    } else {
      None
    }
  }

  pub fn len(&self) -> usize {
    self.0.as_ref().map(|node| 1 + node.tail.len()).unwrap_or(0)
  }

  pub fn is_nil(&self) -> bool {
    self.0.is_none()
  }

  pub fn into_iter(self) -> IntoIter {
    IntoIter(self)
  }
}

pub struct IntoIter(List);

impl Iterator for IntoIter {
  type Item = Expr;

  fn next(&mut self) -> Option<Expr> {
    if self.0.is_nil() {
      return None;
    }

    let (head, tail) = self.0.decons().unwrap();

    self.0 = tail;

    Some(head)
  }
}
