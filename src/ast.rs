#[derive(Debug)]
pub enum Value {
  List(Box<List>),
  Add,
  Number(f64),
  Symbol(u64),
}

#[derive(Debug)]
pub struct List {
  pub head: Value,
  pub tail: Vec<Value>,
}
