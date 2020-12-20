use std::fs;

use zuko::ast::{Atom, Expression};
use zuko::{eval, read};

#[test]
pub fn fibonacci() {
  let source = fs::read_to_string("tests/fibonacci.zuko").unwrap();

  let read_expr = read::read(source.chars()).unwrap();
  let eval_expr = eval::eval(read_expr).unwrap();

  assert_eq!(eval_expr, Expression::Atom(Atom::Float(6765.0)))
}

#[test]
pub fn fizz_buzz() {
  let source = fs::read_to_string("tests/fizz-buzz.zuko").unwrap();

  let read_expr = read::read(source.chars()).unwrap();
  let eval_expr = eval::eval(read_expr).unwrap();

  assert_eq!(eval_expr, Expression::Atom(Atom::String("FizzBuzz".into())))
}

#[test]
pub fn square_root() {
  let source = fs::read_to_string("tests/square-root.zuko").unwrap();

  let read_expr = read::read(source.chars()).unwrap();
  let eval_expr = eval::eval(read_expr).unwrap();

  assert_eq!(eval_expr, Expression::Atom(Atom::Float(2.0000000929222947)))
}
