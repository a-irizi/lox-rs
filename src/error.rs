use std::{rc::Rc, sync::Arc};

use miette::{LabeledSpan, NamedSource, SourceCode, miette};

use crate::scanner;

pub trait IntoReport {
  fn into_report(self, src: Arc<String>) -> miette::Report;
}

impl IntoReport for scanner::Error<'_> {
  fn into_report(self, src: Arc<String>) -> miette::Report {
    match self {
      scanner::Error::InvalidCharacter { character, .. } => miette!(
        labels = vec![LabeledSpan::at(
          (character.as_ptr() as usize - src.as_ptr() as usize, character.len()),
          "this character here"
        )],
        "invalid character"
      )
      .with_source_code(src),
      scanner::Error::UnterminatedString { start, .. } => miette!(
        labels = vec![LabeledSpan::at(
          (start.as_ptr() as usize - src.as_ptr() as usize, start.len()),
          "string literal starts here"
        )],
        "unterminated string literal"
      )
      .with_source_code(src),
      scanner::Error::MissingBlockCommentTerminator { start, .. } => miette!(
        labels = vec![LabeledSpan::at(
          src.as_ptr() as usize - start.as_ptr() as usize,
          "block comment starts here"
        )],
        "unterminated block comment"
      )
      .with_source_code(src),
    }
  }
}

pub fn report(error: impl IntoReport, src: Arc<String>) {
  eprintln!("{:?}", error.into_report(src))
}
