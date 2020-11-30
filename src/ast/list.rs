use std::fmt;
use std::iter::Iterator;
use std::rc::Rc;

use super::Expr;

#[derive(Clone, Debug)]
pub enum List {
  Cons(Rc<Node>),
  Nil,
}

#[derive(Clone, Debug)]
pub struct Node {
  pub head: Expr,
  pub tail: List,
}

impl List {
  pub fn cons(head: Expr, tail: List) -> List {
    use List::*;

    let node = Node { head, tail };
    Cons(Rc::new(node))
  }

  pub fn get(&self, index: usize) -> Option<&Expr> {
    use List::*;

    let node = match self {
      Cons(node) => node,
      Nil => return None,
    };

    if index == 0 {
      Some(&node.head)
    } else {
      node.tail.get(index - 1)
    }
  }

  pub fn len(&self) -> usize {
    use List::*;

    match self {
      Cons(node) => 1 + node.tail.len(),
      Nil => 0,
    }
  }

  pub fn into_iter(self) -> IntoIter {
    IntoIter(self)
  }
}

impl PartialEq for List {
  fn eq(&self, other: &List) -> bool {
    use List::*;

    match (self, other) {
      (Cons(left), Cons(right)) => Rc::ptr_eq(&left, &right),
      (Nil, Nil) => true,
      _ => false,
    }
  }
}

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use List::*;

    if let Nil = self {
      return write!(f, "()");
    }

    write!(
      f,
      "({})",
      self
        .clone()
        .into_iter()
        .map(|expr| format!("{}", expr))
        .collect::<Vec<String>>()
        .join(" ")
    )
  }
}

pub struct IntoIter(List);

impl Iterator for IntoIter {
  type Item = Expr;

  fn next(&mut self) -> Option<Expr> {
    use List::*;

    let node = match &self.0 {
      Cons(node) => node.as_ref(),
      Nil => return None,
    };

    let head = node.head.clone();
    let tail = node.tail.clone();

    self.0 = tail;

    Some(head)
  }
}
