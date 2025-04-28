use thiserror::Error;

use crate::token::Token;

pub type Result<'src, T> = core::result::Result<T, Error<'src>>;

#[derive(Debug, Error)]
pub enum Error<'src> {
  #[error("invalid unary operator token")]
  InvalidUnaryOperator(Token<'src>),
  #[error("invalid binary operator token")]
  InvalidBinaryOperator(Token<'src>),
}
