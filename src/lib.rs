#![warn(missing_docs)]
//! Rox (reads like rocks), is a Lox implementation in the Rust programming language.
//! Lox is defined in the book "Crafting Interpreters" by Robert Nystrom.

mod error;
mod expr;
mod parser;
mod reporting;
mod scanner;
mod token;

use std::{
  io::{Write, stdin, stdout},
  path::Path,
  sync::Arc,
};

use error::report;
use scanner::Scanner;

/// Run a file.
///
/// this will stop at the first error encountered.
///
/// # Arguments
/// * `file_path` - The path to the file to run.
pub fn run_file(_file_path: impl AsRef<Path>) {
  todo!()
}

/// Run a prompt.
///
/// this will stop when user types `Ctrl+D`.
///
/// # Panics
/// This function will panic if failed to flush the output.
pub fn run_prompt() {
  print!("> ");
  stdout().flush().expect("could not flush stdout");
  let mut input = String::new();
  while stdin().read_line(&mut input).is_ok() && input != "q" {
    run(input.clone());
    print!("> ");
    stdout().flush().expect("could not flush stdout");
    input.clear();
  }
}

fn run(content: String) {
  let content = Arc::new(content);
  Scanner::new(content.as_str())
    .filter_map(|n| match n {
      Ok(t) => Some(t),
      Err(e) => {
        report(e, Arc::clone(&content));
        None
      }
    })
    .for_each(|token| match &token.kind {
      token::TokenKind::String(literal) => {
        println!("String {} {}", token.lexeme.unwrap_or_default(), literal);
      }
      token::TokenKind::Number(literal) => {
        println!("Number {} {}", token.lexeme.unwrap_or_default(), literal);
      }
      _ => println!("{:?} {}", token.kind, token.lexeme.unwrap_or_default()),
    });
}
