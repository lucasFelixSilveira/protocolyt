use std::process;
use colored::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
  IOe,
  Lexical,
  TryAgain,
  InvalidOperation,
}

pub fn spawn(error_type: Error, error: String, complement: Option<String>) -> Result<(), ()> {

  let code: (i32, &'static str) = match error_type {
    Error::IOe => (5, "IO Error"),
    Error::Lexical => (84, "Lexical Error"),
    Error::TryAgain => (11, "Try again"),
    Error::InvalidOperation => (1, "Invalid operation"),
  };

  let error_status: ColoredString = format!("[{} - {}]", code.0, code.1).red();

  println!("{error_status}: {}", [
    String::from(error), 
    match complement {
      None => String::new(),
      Some(complement) => ["\n".to_string(), complement].concat()
    }
  ].concat());

  process::exit(0)
}