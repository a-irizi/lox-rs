use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error("invalid unary operator token")]
  InvalidUnaryOperator,
  #[error("invalid binary operator token")]
  InvalidBinaryOperator,
}
