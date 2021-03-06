use std::iter::{Iterator, Peekable};

use thiserror::Error;

use crate::ast::{self, Atom, Expr, List, Operator, Special, Symbol};

pub fn read(source: &str) -> Result<Expr, ReadError> {
  use List::*;

  let mut reader = Reader::new(source.chars());

  let mut exprs = vec![];

  loop {
    exprs.push(reader.read_expr()?);
    if reader.is_empty() {
      break;
    }
  }

  exprs.reverse();
  let mut list = Nil;
  for expr in exprs.into_iter() {
    list = List::cons(expr, list);
  }

  Ok(Expr::List(List::cons(
    Expr::Atom(Atom::Special(Special::Begin)),
    list,
  )))
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

  pub fn is_empty(&mut self) -> bool {
    self.source.peek().is_none()
  }

  pub fn read_expr(&mut self) -> Result<Expr, ReadError> {
    use Expr::*;
    use ReadError::*;

    self.skip_whitespace_or_comment();

    let expr = match self.source.peek() {
      Some('(') => List(self.read_list()?),
      Some(_) => Atom(self.read_atom()?),
      None => return Err(UnexpectedEndOfInput),
    };

    self.skip_whitespace_or_comment();

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
    self.skip_whitespace_or_comment();
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
    use ast::Special::Operator;
    use Atom::*;
    use ReadError::*;

    let atom = match self.source.peek() {
      Some('"') => String(self.read_string()?),
      Some(char) if char.is_digit(10) => Number(self.read_number()?),
      Some(char) if is_operator(*char) => {
        Special(Operator(self.read_operator()?))
      }
      Some(char) if is_symbol(*char) => self.read_symbol_or_special()?,
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

  pub fn read_symbol_or_special(&mut self) -> Result<Atom, ReadError> {
    use Special::*;

    let symbol = self.read_symbol()?;

    let special = match symbol.as_str() {
      "begin" => Begin,
      "define" => Define,
      "function" => Function,
      "macro" => Macro,
      "if" => If,
      "quote" => Quote,
      _ => return Ok(Atom::Symbol(symbol)),
    };

    Ok(Atom::Special(special))
  }

  pub fn read_symbol(&mut self) -> Result<Symbol, ReadError> {
    use ReadError::*;

    let mut buf = Vec::new();
    let mut should_break = false;
    let mut prev_punct_dist = 0;

    loop {
      match self.source.peek() {
        Some(')') => break,
        Some(char) if char.is_whitespace() => break,
        Some(char) if should_break => return Err(UnexpectedChar(*char)),
        Some(char) if char.is_alphabetic() && char.is_lowercase() => {}
        Some('-') | Some('/') if prev_punct_dist > 0 => {
          prev_punct_dist = -1;
        }
        Some('?') => should_break = true,
        Some(char) => return Err(UnexpectedChar(*char)),
        None => break,
      }
      let char = self.source.next().unwrap();

      buf.push(char);
      prev_punct_dist += 1;
    }

    let last_char = buf.last().cloned();
    if let Some('-') | Some('/') = last_char {
      return Err(UnexpectedChar(last_char.unwrap()));
    }

    let buf: String = buf.into_iter().collect();

    Ok(Symbol::new(buf))
  }

  pub fn read_operator(&mut self) -> Result<Operator, ReadError> {
    use Operator::*;
    use ReadError::*;

    let operator = match self.source.peek() {
      Some('+') => Add,
      Some('-') => Sub,
      Some('*') => Mul,
      Some('/') => Div,
      Some('%') => Mod,
      Some('>') => Gt,
      Some('<') => Lt,
      Some('=') => Eq,
      Some(char) => return Err(UnexpectedChar(*char)),
      None => return Err(UnexpectedEndOfInput),
    };
    self.source.next();

    Ok(operator)
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

  pub fn skip_whitespace_or_comment(&mut self) {
    loop {
      match self.source.peek() {
        Some(';') => self.skip_comment(),
        Some(char) if !char.is_whitespace() => break,
        None => break,
        _ => {}
      }
      self.source.next();
    }
  }

  pub fn skip_comment(&mut self) {
    match self.source.peek() {
      Some(';') => {}
      Some(_) => return,
      None => return,
    };
    self.source.next();

    loop {
      match self.source.peek() {
        Some('\n') => break,
        Some(_) => {}
        None => return,
      }
      self.source.next();
    }

    // Get rid of final newline.
    self.source.next();
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
    '+' | '-' | '*' | '/' | '%' | '>' | '<' | '=' => true,
    _ => false,
  }
}
