use std::rc::Rc;

use internment::Intern;

use super::env::Frame;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
  Atom(Atom),
  List(List),
}

impl Expression {
  pub fn is_truthy(&self) -> bool {
    if let Expression::List(List::Nil) = self {
      false
    } else {
      true
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
  Int(i64),
  Float(f64),
  String(String),
  Symbol(Symbol),
  Function(Function),
  Macro(Macro),
  Special(Special), // special form
  Native(Native),   // native function
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Symbol(Intern<String>);

impl Symbol {
  pub fn new<S>(symbol: S) -> Symbol
  where
    S: Into<String>,
  {
    Symbol(Intern::new(symbol.into()))
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function(Rc<FunctionInner>);

impl Function {
  pub fn new(
    frame: Frame,
    parameters: Vec<Symbol>,
    body: Expression,
  ) -> Function {
    Function(Rc::new(FunctionInner {
      frame,
      parameters,
      body,
    }))
  }

  pub fn frame(&self) -> &Frame {
    &self.0.frame
  }

  pub fn parameters(&self) -> &[Symbol] {
    &self.0.parameters
  }

  pub fn body(&self) -> &Expression {
    &self.0.body
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionInner {
  pub frame: Frame,
  pub parameters: Vec<Symbol>,
  pub body: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Macro(Rc<MacroInner>);

impl Macro {
  pub fn new(parameter: Symbol, body: Expression) -> Macro {
    Macro(Rc::new(MacroInner { parameter, body }))
  }

  pub fn parameter(&self) -> &Symbol {
    &self.0.parameter
  }

  pub fn body(&self) -> &Expression {
    &self.0.body
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MacroInner {
  pub parameter: Symbol,
  pub body: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Special {
  Do,
  Define,
  Function,
  Macro,
  If,
  Quote,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Native(Rc<NativeFunction>);

impl Native {
  pub fn new(function: NativeFunction) -> Native {
    Native(Rc::new(function))
  }
}

pub type NativeFunction =
  fn(arguments: Vec<Expression>) -> Result<Expression, crate::eval::EvalError>;

#[derive(Clone, Debug, PartialEq)]
pub enum List {
  Cons(Cons),
  Nil,
}

impl List {
  pub fn cons(car: Expression, cdr: List) -> List {
    List::Cons(Cons {
      car: Rc::new(car),
      cdr: Rc::new(cdr),
    })
  }

  pub fn get(&self, index: usize) -> Option<&Expression> {
    use List::*;

    let node = match self {
      Cons(node) => node,
      Nil => return None,
    };

    if index == 0 {
      Some(&node.car)
    } else {
      node.cdr.get(index - 1)
    }
  }

  pub fn len(&self) -> usize {
    use List::*;

    match self {
      Cons(node) => 1 + node.cdr.len(),
      Nil => 0,
    }
  }

  pub fn push(&self, expression: Expression) -> List {
    match self {
      List::Cons(cons) => List::Cons(Cons {
        car: cons.car.clone(),
        cdr: Rc::new(cons.cdr.push(expression)),
      }),
      List::Nil => List::cons(expression, List::Nil),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cons {
  pub car: Rc<Expression>,
  pub cdr: Rc<List>,
}
