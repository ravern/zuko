use std::fs;

use zuko::ast::{Atom, Expr};
use zuko::{eval, read};

#[test]
pub fn fibonacci() {
  let source = fs::read_to_string("tests/fibonacci.zuko").unwrap();

  let read_expr = read::read(&source).unwrap();
  let eval_expr = eval::eval(read_expr).unwrap();

  assert_eq!(eval_expr, Expr::Atom(Atom::Number(6765.0)))
}

#[test]
pub fn fizz_buzz() {
  let source = fs::read_to_string("tests/fizz-buzz.zuko").unwrap();

  let read_expr = read::read(&source).unwrap();
  let eval_expr = eval::eval(read_expr).unwrap();

  assert_eq!(eval_expr, Expr::Atom(Atom::String("FizzBuzz".into())))
}
