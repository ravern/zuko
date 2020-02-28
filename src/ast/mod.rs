use std::fmt;
use std::rc::Rc;

use crate::env::Frame;

pub use self::list::{List, Node};

pub mod list;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
  List(List),
  Atom(Atom),
}

impl Expr {
  pub fn is_truthy(&self) -> bool {
    if let Expr::List(List::Nil) = self {
      false
    } else {
      true
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
  Number(f64),
  Symbol(Symbol),
  String(String),
  Function(Function),
  Native(Native),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Symbol {
  inner: Rc<String>,
}

impl Symbol {
  pub fn new(symbol: String) -> Symbol {
    Symbol {
      inner: Rc::new(symbol),
    }
  }

  pub fn as_str(&self) -> &str {
    self.inner.as_ref()
  }
}

#[derive(Clone)]
pub struct Function {
  inner: Rc<FunctionInner>,
}

pub struct FunctionInner {
  pub frame: Frame,
  pub parameters: Vec<Symbol>,
  pub body: Expr,
}

impl Function {
  pub fn new(frame: Frame, parameters: Vec<Symbol>, body: Expr) -> Function {
    Function {
      inner: Rc::new(FunctionInner {
        frame,
        parameters,
        body,
      }),
    }
  }

  pub fn frame(&self) -> &Frame {
    &self.inner.frame
  }

  pub fn parameters(&self) -> &[Symbol] {
    &self.inner.parameters
  }

  pub fn body(&self) -> &Expr {
    &self.inner.body
  }
}

impl PartialEq for Function {
  fn eq(&self, other: &Function) -> bool {
    Rc::ptr_eq(&self.inner, &other.inner)
  }
}

impl fmt::Debug for Function {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Function {{ parameters: {:?}, body: {:?} }}",
      self.inner.parameters, self.inner.body
    )
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Native {
  Begin,
  Define,
  Function,
  If,
  Quote,
  Operator(Operator),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
  Eq,
}
