use std::{fmt::Display, ops::Deref};

use crate::{
  expr::error,
  token::{Token, TokenKind},
};

/// represents a unary operator token.
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
  type Error = error::Error<'src>;

  fn try_from(token: Token<'src>) -> error::Result<'src, Self> {
    match token.kind {
      TokenKind::Bang | TokenKind::Minus => Ok(UnaryOperator(token)),
      _ => Err(error::Error::UnaryOperator(token)),
    }
  }
}
