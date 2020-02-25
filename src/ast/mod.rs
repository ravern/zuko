use std::fmt;
use std::rc::Rc;

use crate::env::Frame;

pub use self::list::{List, Node};

pub mod list;

#[derive(Clone, Debug)]
pub enum Expr {
  List(List),
  Atom(Atom),
}

#[derive(Clone, Debug)]
pub enum Atom {
  Number(f64),
  Symbol(String),
  String(String),
  Function(Function),
  Native(Native),
}

#[derive(Clone)]
pub struct Function {
  inner: Rc<FunctionInner>,
}

pub struct FunctionInner {
  pub frame: Frame,
  pub parameters: Vec<String>,
  pub body: Expr,
}

impl Function {
  pub fn new(frame: Frame, parameters: Vec<String>, body: Expr) -> Function {
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

  pub fn parameters(&self) -> &[String] {
    &self.inner.parameters
  }

  pub fn body(&self) -> &Expr {
    &self.inner.body
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

#[derive(Clone, Debug)]
pub enum Native {
  Begin,
  Define,
  Function,
  Quote,
  Operator(Operator),
}

#[derive(Clone, Debug)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
}
