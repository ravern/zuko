use std::iter::{Iterator, Peekable};

use thiserror::Error;

use crate::ast::{Atom, Expr, List};

pub fn read(source: &str) -> Result<Expr, ReadError> {
  let mut reader = Reader::new(source.chars());
  reader.read_expr()
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

  pub fn read_expr(&mut self) -> Result<Expr, ReadError> {
    use Expr::*;
    use ReadError::*;

    self.skip_whitespace();

    let expr = match self.source.peek() {
      Some('(') => List(self.read_list()?),
      Some(_) => Atom(self.read_atom()?),
      None => return Err(UnexpectedEndOfInput),
    };

    self.skip_whitespace();

    Ok(expr)
  }

  pub fn read_list(&mut self) -> Result<List, ReadError> {
    use List::*;
    use ReadError::*;

    match self.source.peek() {
      Some('(') => {}
      Some(char) => return Err(UnexpectedChar(*char)),
      None => return Err(UnexpectedEndOfInput),
    }
    self.source.next();

    // Catch empty lists
    self.skip_whitespace();
    if let Some(')') = self.source.peek() {
      self.source.next();
      return Ok(List::Nil);
    }

    let mut exprs = vec![self.read_expr()?];

    loop {
      match self.source.peek() {
        Some(')') => {
          self.source.next();
          break;
        }
        None => return Err(UnexpectedEndOfInput),
        _ => {}
      }

      exprs.push(self.read_expr()?);
    }

    exprs.reverse();
    let mut list = Nil;
    for expr in exprs.into_iter() {
      list = List::cons(expr, list);
    }

    Ok(list)
  }

  pub fn read_atom(&mut self) -> Result<Atom, ReadError> {
    use Atom::*;
    use ReadError::*;

    let atom = match self.source.peek() {
      Some('"') => String(self.read_string()?),
      Some(char) if char.is_digit(10) => Number(self.read_number()?),
      Some(char) if is_symbol(*char) => Symbol(self.read_symbol()?),
      Some(char) if is_operator(*char) => Symbol(self.read_operator_symbol()?),
      Some(char) => return Err(UnexpectedChar(*char)),
      None => return Err(UnexpectedEndOfInput),
    };

    Ok(atom)
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

  pub fn read_symbol(&mut self) -> Result<String, ReadError> {
    use ReadError::*;

    let mut buf = Vec::new();
    let mut prev_hyphen_dist = 0;

    loop {
      match self.source.peek() {
        Some(char) if char.is_alphabetic() && char.is_lowercase() => {}
        Some('-') if prev_hyphen_dist > 0 => {
          prev_hyphen_dist = -1;
        }
        Some(')') => break,
        Some(char) if char.is_whitespace() => break,
        Some(char) => return Err(UnexpectedChar(*char)),
        None => break,
      }
      let char = self.source.next().unwrap();

      buf.push(char);
      prev_hyphen_dist += 1;
    }

    let buf: String = buf.into_iter().collect();

    Ok(buf)
  }

  pub fn read_operator_symbol(&mut self) -> Result<String, ReadError> {
    use ReadError::*;

    let operator = match self.source.peek() {
      Some('+') => "+",
      Some('-') => "-",
      Some('*') => "*",
      Some('/') => "/",
      Some(char) => return Err(UnexpectedChar(*char)),
      None => return Err(UnexpectedEndOfInput),
    };
    self.source.next();

    Ok(operator.to_string())
  }

  pub fn read_string(&mut self) -> Result<String, ReadError> {
    use ReadError::*;

    match self.source.peek() {
      Some('"') => {}
      Some(char) => return Err(UnexpectedChar(*char)),
      None => return Err(UnexpectedEndOfInput),
    }
    self.source.next();

    let mut buf = Vec::new();

    loop {
      match self.source.peek() {
        Some('"') => break,
        Some(_) => {}
        None => return Err(UnexpectedEndOfInput),
      }
      let char = self.source.next().unwrap();

      buf.push(char);
    }

    let buf: String = buf.into_iter().collect();

    // Get rid of final quote.
    self.source.next();

    Ok(buf)
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

fn is_symbol(char: char) -> bool {
  char.is_alphabetic() && char.is_lowercase()
}

fn is_operator(char: char) -> bool {
  match char {
    '+' | '-' | '*' | '/' => true,
    _ => false,
  }
}
