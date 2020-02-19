use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use thiserror::Error;

pub fn run() -> Result<(), RunError> {
  println!("Yu v0.1.0");

  let mut editor = Editor::<()>::new();
  editor.set_auto_add_history(true);

  loop {
    match editor.readline("> ") {
      Ok(line) => {
        println!("{}", line);
      }
      Err(ReadlineError::Interrupted) => break,
      Err(ReadlineError::Eof) => break,
      Err(_) => println!("error: failed to read line"),
    }
  }
  println!("Bye!");

  Ok(())
}

#[derive(Debug, Error)]
pub enum RunError {}
