use std::{fmt::Display, ops::Deref};

use crate::token::{Token, TokenKind};

pub struct UnaryOperator<'src>(Token<'src>);

impl<'src> AsRef<Token<'src>> for UnaryOperator<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl<'src> Deref for UnaryOperator<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for UnaryOperator<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl<'src> TryFrom<Token<'src>> for UnaryOperator<'src> {
  type Error = super::Error;

  fn try_from(value: Token<'src>) -> super::Result<UnaryOperator<'src>> {
    match value.kind {
      TokenKind::Bang | TokenKind::Minus => Ok(UnaryOperator(value)),
      _ => Err(super::Error::InvalidUnaryOperator),
    }
  }
}
