use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

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

lazy_static! {
  static ref SYMBOLS: Mutex<Vec<Symbol>> = Mutex::new(Vec::new());
  pub static ref SYMBOL_TRUE: Symbol = Symbol::new("true");
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Symbol {
  inner: Arc<String>,
}

impl Symbol {
  pub fn new<S>(symbol: S) -> Symbol
  where
    S: Into<String>,
  {
    let symbol = symbol.into();

    if let Some(symbol) = SYMBOLS
      .lock()
      .unwrap()
      .iter()
      .find(|s| s.inner.as_ref() == &symbol)
    {
      return symbol.clone();
    }

    let symbol = Symbol {
      inner: Arc::new(symbol),
    };
    SYMBOLS.lock().unwrap().push(symbol.clone());
    symbol
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
  Debug,
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
