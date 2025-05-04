use thiserror::Error;

use crate::token::Token;

pub type Result<'src, T> = core::result::Result<T, Error<'src>>;

#[derive(Debug, Error)]
pub enum Error<'src> {
  #[error("invalid unary operator token")]
  UnaryOperator(Token<'src>),
  #[error("invalid binary operator token")]
  BinaryOperator(Token<'src>),
  #[error("invalid comparison operator token")]
  ComparisonOperator(Token<'src>),
  #[error("invalid factor operator token")]
  FactorOperator(Token<'src>),
  #[error("invalid term operator token")]
  TermOperator(Token<'src>),
  #[error("invalid comma operator token")]
  CommaOperator(Token<'src>),
  #[error("invalid ternary then branch token")]
  TernaryThenOperator(Token<'src>),
  #[error("invalid ternary else branch token")]
  TernaryElseOperator(Token<'src>),
  #[error("invalid literal token")]
  Literal(Token<'src>),
}
