#![warn(missing_docs)]
//! Rox (reads like rocks), is a Lox implementation in the Rust programming language.
//! Lox is defined in the book "Crafting Interpreters" by Robert Nystrom.

mod error;
mod expr;
mod parser;
mod reporting;
mod scanner;
mod token;

use std::path::Path;

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
    todo!()
}

fn run(_content: String) {
    todo!()
}
