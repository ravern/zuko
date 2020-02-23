use std::collections::BTreeMap;
use std::iter::Iterator;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Expr {
  List(List),
  Atom(Atom),
}

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

#[derive(Clone, Debug)]
pub enum Atom {
  Number(f64),
  Symbol(String),
  String(String),
  Function(Box<Function>),
  Native(Native),
}

#[derive(Clone, Debug)]
pub struct Function {
  pub scope: Scope,
  pub parameters: Vec<String>,
  pub body: Expr,
}

#[derive(Clone, Debug)]
pub enum Native {
  Begin,
  Define,
  Function,
  Quote,
}

#[derive(Clone, Debug)]
pub struct Scope {
  variables: BTreeMap<String, Expr>,
}

impl Scope {
  pub fn new() -> Scope {
    Scope {
      variables: BTreeMap::new(),
    }
  }

  pub fn get(&self, symbol: &str) -> Option<Expr> {
    self.variables.get(symbol).cloned()
  }

  pub fn set(&mut self, symbol: String, expr: Expr) {
    self.variables.insert(symbol, expr);
  }
}
