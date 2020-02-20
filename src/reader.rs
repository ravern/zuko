use std::iter::{Iterator, Peekable};

use thiserror::Error;

use crate::ast::{Expr, List};

pub fn read(source: &str) -> Result<Expr, ReadError> {
  let mut reader = Reader::new(source.chars());
  reader.read_value()
}

pub struct Reader<I>
where
  I: Iterator<Item = char>,
{
  source: Peekable<I>,
}

impl<I> Reader<I>
where
  I: Iterator<Item = char>,
{
  pub fn new(source: I) -> Reader<I> {
    Reader {
      source: source.peekable(),
    }
  }

  pub fn read_value(&mut self) -> Result<Expr, ReadError> {
    use Expr::*;
    use ReadError::*;

    self.skip_whitespace();

    let value = match self.source.peek() {
      Some('(') => List(Box::new(self.read_list()?)),
      Some(char) if char.is_digit(10) => Number(self.read_number()?),
      Some('+') => {
        self.source.next();
        Add
      }
      Some(char) => return Err(UnexpectedChar(*char)),
      None => return Err(UnexpectedEndOfInput),
    };

    self.skip_whitespace();

    Ok(value)
  }

  pub fn read_list(&mut self) -> Result<List, ReadError> {
    use ReadError::*;

    match self.source.peek() {
      Some('(') => {}
      Some(char) => return Err(UnexpectedChar(*char)),
      None => return Err(UnexpectedEndOfInput),
    }
    self.source.next();

    let head = self.read_value()?;

    let mut tail = Vec::new();
    loop {
      match self.source.peek() {
        Some(')') => {
          self.source.next();
          break;
        }
        None => return Err(UnexpectedEndOfInput),
        _ => {}
      }

      tail.push(self.read_value()?);
    }

    Ok(List { head, tail })
  }

  pub fn read_number(&mut self) -> Result<f64, ReadError> {
    use ReadError::*;

    let mut buf = Vec::new();
    let mut has_decimal = false;

    loop {
      match self.source.peek() {
        Some('.') if !has_decimal => has_decimal = true,
        Some(char) if char.is_digit(10) => {}
        Some(')') => break,
        Some(char) if char.is_whitespace() => break,
        Some(char) => return Err(UnexpectedChar(*char)),
        None => break,
      }
      let char = self.source.next().unwrap();

      buf.push(char);
    }

    let buf: String = buf.into_iter().collect();
    let number = buf.parse().unwrap();

    Ok(number)
  }

  pub fn skip_whitespace(&mut self) {
    loop {
      match self.source.peek() {
        Some(char) if !char.is_whitespace() => break,
        None => break,
        _ => {}
      }
      self.source.next();
    }
  }
}

#[derive(Debug, Error)]
pub enum ReadError {
  #[error("unexpected end of input")]
  UnexpectedEndOfInput,
  #[error("unexpected char '{0}'")]
  UnexpectedChar(char),
}
