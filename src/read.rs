use std::iter::Peekable;

use thiserror::Error;

use crate::ast::{Atom, Expression, List, Special, Symbol};

pub fn read<S>(source: S) -> Result<Expression, ReadError>
where
  S: Iterator<Item = char>,
{
  let mut reader = Reader::new(source);
  Ok(Expression::List(List::cons(
    Expression::Atom(Atom::Special(Special::Do)),
    reader.expressions_to_end()?,
  )))
}

pub struct Reader<S>
where
  S: Iterator<Item = char>,
{
  source: Peekable<S>,
}

impl<S> Reader<S>
where
  S: Iterator<Item = char>,
{
  pub fn new(source: S) -> Reader<S> {
    Reader {
      source: source.peekable(),
    }
  }

  pub fn expressions_to_end(&mut self) -> Result<List, ReadError> {
    let mut list = List::Nil;

    loop {
      self.whitespace();
      match self.source.peek() {
        Some(_) => {}
        None => break,
      }
      list = list.push(self.expression()?);
    }

    Ok(list)
  }

  pub fn expression(&mut self) -> Result<Expression, ReadError> {
    match self.source.peek() {
      Some('(') => Ok(Expression::List(self.list()?)),
      Some(_) => Ok(Expression::Atom(self.atom()?)),
      None => Err(ReadError::UnexpectedEof),
    }
  }

  pub fn list(&mut self) -> Result<List, ReadError> {
    match self.source.next() {
      Some('(') => {}
      Some(char) => return Err(ReadError::UnexpectedChar(char)),
      None => return Err(ReadError::UnexpectedEof),
    }

    let mut list = List::Nil;

    loop {
      self.whitespace();
      match self.source.peek() {
        Some(')') => {
          self.source.next().unwrap();
          break;
        }
        Some(_) => {}
        None => return Err(ReadError::UnexpectedEof),
      }
      list = list.push(self.expression()?);
    }

    Ok(list)
  }

  pub fn atom(&mut self) -> Result<Atom, ReadError> {
    match self.source.peek() {
      Some('"') => Ok(Atom::String(self.string()?)),
      Some(char) if char.is_digit(10) => Ok(self.int_or_float()?),
      Some(_) => Ok(Atom::Symbol(self.symbol()?)),
      None => Err(ReadError::UnexpectedEof),
    }
  }

  pub fn string(&mut self) -> Result<String, ReadError> {
    match self.source.next() {
      Some('"') => {}
      Some(char) => return Err(ReadError::UnexpectedChar(char)),
      None => return Err(ReadError::UnexpectedEof),
    }

    let mut string = vec![];

    loop {
      match self.source.peek() {
        Some('"') => {
          self.source.next().unwrap();
          break;
        }
        Some(_) => {}
        None => return Err(ReadError::UnexpectedEof),
      }
      string.push(self.source.next().unwrap());
    }

    Ok(string.into_iter().collect())
  }

  pub fn int_or_float(&mut self) -> Result<Atom, ReadError> {
    let mut is_float = false;
    let mut int_or_float = vec![];

    loop {
      match self.source.peek() {
        Some(char) if is_terminal(*char) => break,
        Some('.') if is_float => return Err(ReadError::UnexpectedChar('.')),
        Some('.') => is_float = true,
        Some(char) if char.is_digit(10) => {}
        Some(char) => return Err(ReadError::UnexpectedChar(*char)),
        None => break,
      }
      int_or_float.push(self.source.next().unwrap());
    }

    if is_float {
      Ok(Atom::Float(
        int_or_float
          .into_iter()
          .collect::<String>()
          .parse()
          .unwrap(),
      ))
    } else {
      Ok(Atom::Int(
        int_or_float
          .into_iter()
          .collect::<String>()
          .parse()
          .unwrap(),
      ))
    }
  }

  pub fn symbol(&mut self) -> Result<Symbol, ReadError> {
    let mut symbol = vec![];

    loop {
      match self.source.peek() {
        Some(char) if is_terminal(*char) => break,
        Some(_) => {}
        None if !symbol.is_empty() => break,
        None => return Err(ReadError::UnexpectedEof),
      }
      symbol.push(self.source.next().unwrap());
    }

    Ok(Symbol::new(symbol.into_iter().collect::<String>()))
  }

  pub fn whitespace(&mut self) {
    loop {
      match self.source.peek() {
        Some(char) if char.is_whitespace() => {}
        _ => break,
      }
      self.source.next();
    }
  }
}

fn is_terminal(char: char) -> bool {
  match char {
    char if char.is_whitespace() => true,
    ')' => true,
    _ => false,
  }
}

#[derive(Debug, Error)]
pub enum ReadError {
  #[error("unexpected end of file")]
  UnexpectedEof,
  #[error("unexpected '{0}'")]
  UnexpectedChar(char),
}

#[cfg(test)]
mod tests {
  use crate::{
    ast::{Atom, Expression, List, Special, Symbol},
    read::read,
  };

  #[test]
  fn int() {
    assert_eq!(
      read("1234".chars()).unwrap(),
      Expression::List(List::cons(
        Expression::Atom(Atom::Special(Special::Do)),
        List::cons(Expression::Atom(Atom::Int(1234)), List::Nil)
      ))
    );
  }

  #[test]
  fn float() {
    assert_eq!(
      read("12.34".chars()).unwrap(),
      Expression::List(List::cons(
        Expression::Atom(Atom::Special(Special::Do)),
        List::cons(Expression::Atom(Atom::Float(12.34)), List::Nil)
      ))
    );
  }

  #[test]
  fn string() {
    assert_eq!(
      read("\"Hello, world!\"".chars()).unwrap(),
      Expression::List(List::cons(
        Expression::Atom(Atom::Special(Special::Do)),
        List::cons(
          Expression::Atom(Atom::String("Hello, world!".into())),
          List::Nil
        )
      ))
    );
  }

  #[test]
  fn symbol() {
    assert_eq!(
      read("test".chars()).unwrap(),
      Expression::List(List::cons(
        Expression::Atom(Atom::Special(Special::Do)),
        List::cons(
          Expression::Atom(Atom::Symbol(Symbol::new("test"))),
          List::Nil
        )
      ))
    );
  }

  #[test]
  fn list() {
    assert_eq!(
      read("(1 2 3 4)".chars()).unwrap(),
      Expression::List(List::cons(
        Expression::Atom(Atom::Special(Special::Do)),
        List::cons(
          Expression::List(List::cons(
            Expression::Atom(Atom::Int(1)),
            List::cons(
              Expression::Atom(Atom::Int(2)),
              List::cons(
                Expression::Atom(Atom::Int(3)),
                List::cons(Expression::Atom(Atom::Int(4)), List::Nil)
              )
            )
          )),
          List::Nil
        )
      ))
    );
  }

  #[test]
  fn nested_lists() {
    assert_eq!(
      read("(1 2 (3 4))".chars()).unwrap(),
      Expression::List(List::cons(
        Expression::Atom(Atom::Special(Special::Do)),
        List::cons(
          Expression::List(List::cons(
            Expression::Atom(Atom::Int(1)),
            List::cons(
              Expression::Atom(Atom::Int(2)),
              List::cons(
                Expression::List(List::cons(
                  Expression::Atom(Atom::Int(3)),
                  List::cons(Expression::Atom(Atom::Int(4)), List::Nil)
                )),
                List::Nil
              )
            )
          )),
          List::Nil
        )
      ))
    );
  }
}
