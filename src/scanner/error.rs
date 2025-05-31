use thiserror::Error;

pub type Result<'src, T> = core::result::Result<T, Error<'src>>;

#[derive(Debug, Error)]
pub enum Error<'src> {
  #[error("invalid character {character} at line {line}")]
  InvalidCharacter { character: &'src str, line: usize },
  #[error("unterminated string at line {line}")]
  UnterminatedString { start: &'src str, line: usize },
  #[error("unterminated block comment at line {line}")]
  MissingBlockCommentTerminator { start: &'src str, line: usize },
}
