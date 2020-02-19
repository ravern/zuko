use std::iter::{Iterator, Peekable};

use thiserror::Error;

use crate::ast::{List, Value};

pub fn read(source: &str) -> Result<Value, ReadError> {
  let mut source = source.chars().peekable();
  read_value(&mut source)
}

#[derive(Debug, Error)]
pub enum ReadError {
  #[error("unexpected end of input")]
  UnexpectedEndOfInput,
  #[error("unexpected char {0}")]
  UnexpectedChar(char),
}

fn read_value<I>(source: &mut Peekable<I>) -> Result<Value, ReadError>
where
  I: Iterator<Item = char>,
{
  use ReadError::*;
  use Value::*;

  skip_whitespace(source);

  let value = match source.peek() {
    Some('(') => List(Box::new(read_list(source)?)),
    Some(char) if char.is_digit(10) => Number(read_number(source)?),
    Some('+') => {
      source.next();
      Add
    }
    Some(char) => return Err(UnexpectedChar(*char)),
    None => return Err(UnexpectedEndOfInput),
  };

  skip_whitespace(source);

  Ok(value)
}

fn read_list<I>(source: &mut Peekable<I>) -> Result<List, ReadError>
where
  I: Iterator<Item = char>,
{
  use ReadError::*;

  match source.peek() {
    Some('(') => {}
    Some(char) => return Err(UnexpectedChar(*char)),
    None => return Err(UnexpectedEndOfInput),
  }
  source.next();

  let head = read_value(source)?;

  let mut tail = Vec::new();
  loop {
    match source.peek() {
      Some(')') => {
        source.next();
        break;
      }
      None => return Err(UnexpectedEndOfInput),
      _ => {}
    }

    tail.push(read_value(source)?);
  }

  Ok(List { head, tail })
}

fn read_number<I>(source: &mut Peekable<I>) -> Result<f64, ReadError>
where
  I: Iterator<Item = char>,
{
  use ReadError::*;

  let mut buf = Vec::new();
  let mut has_decimal = false;

  loop {
    match source.peek() {
      Some('.') if !has_decimal => has_decimal = true,
      Some(char) if char.is_digit(10) => {}
      Some(')') => break,
      Some(char) if char.is_whitespace() => break,
      Some(char) => return Err(UnexpectedChar(*char)),
      None => return Err(UnexpectedEndOfInput),
    }
    let char = source.next().unwrap();

    buf.push(char);
  }

  let buf: String = buf.into_iter().collect();
  let number = buf.parse().unwrap();

  Ok(number)
}

fn skip_whitespace<I>(source: &mut Peekable<I>)
where
  I: Iterator<Item = char>,
{
  loop {
    match source.peek() {
      Some(char) if !char.is_whitespace() => break,
      None => break,
      _ => {}
    }
    source.next();
  }
}
