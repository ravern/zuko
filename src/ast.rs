#[derive(Debug)]
pub enum Expr {
  List(Box<List>),
  Add,
  Number(f64),
  Symbol(String),
  Ident(String),
  String(String),
}

#[derive(Debug)]
pub struct List {
  pub head: Expr,
  pub tail: Vec<Expr>,
}
