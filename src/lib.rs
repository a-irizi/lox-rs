#![warn(missing_docs)]
//! Rox (reads like rocks), is a Lox implementation in the Rust programming language.
//! Lox is defined in the book "Crafting Interpreters" by Robert Nystrom.

mod error;
mod reporting;
mod scanner;
mod token;

use self::error::Result;

use std::path::Path;

/// Run a file.
///
/// this will stop at the first error encountered.
///
/// # Arguments
/// * `file_path` - The path to the file to run.
///
/// # Errors
/// This function will return an error if encountered an error while
/// running the file, else it will return `Ok(())` indicating success.
pub fn run_file(_file_path: impl AsRef<Path>) -> Result<()> {
  todo!()
}

/// Run a prompt.
///
/// this will stop when user types `Ctrl+D`.
///
/// # Panics
/// This function will panic if failed to flush the output.
pub fn run_prompt() {
  todo!()
}

fn run(_content: String) -> Result<()> {
  todo!()
}
