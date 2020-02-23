use std::collections::BTreeMap;

pub use self::list::List;

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
