#[derive(Debug)]
pub enum Expr {
  List(Box<List>),
  Add,
  Number(f64),
  Symbol(u64),
}

#[derive(Debug)]
pub struct List {
  pub head: Expr,
  pub tail: Vec<Expr>,
}
