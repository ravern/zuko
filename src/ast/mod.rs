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
  Function(Box<Function>),
  Native(Native),
}

#[derive(Clone, Debug)]
pub struct Function {
  pub frame: Rc<Frame>,
  pub parameters: Vec<String>,
  pub body: Expr,
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
