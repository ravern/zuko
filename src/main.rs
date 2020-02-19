use std::process;

use yu::run;

fn main() {
  if let Err(error) = run() {
    println!("error: {}", error);
    process::exit(1);
  }
}
