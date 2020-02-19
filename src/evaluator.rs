use thiserror::Error;

use crate::ast::{List, Value};

pub fn eval(value: Value) -> Result<Value, EvalError> {
  use Value::*;

  match value {
    List(list) => eval_list(*list),
    value => Ok(value),
  }
}

#[derive(Debug, Error)]
pub enum EvalError {
  #[error("can only add numbers")]
  OnlyAddNumbers,
  #[error("expression not callable")]
  NotCallable,
}

fn eval_list(list: List) -> Result<Value, EvalError> {
  use Value::*;

  let head = list.head;
  let tail = list.tail;

  match head {
    Add => eval_add(tail),
    _ => Err(EvalError::NotCallable),
  }
}

fn eval_add(args: Vec<Value>) -> Result<Value, EvalError> {
  let arguments = args
    .into_iter()
    .map(|arg| match arg {
      Value::Number(number) => Ok(number),
      _ => Err(EvalError::OnlyAddNumbers),
    })
    .collect::<Result<Vec<f64>, EvalError>>()?;

  Ok(Value::Number(arguments.into_iter().sum()))
}
