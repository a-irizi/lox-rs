use std::{fmt::Display, ops::Deref};

use crate::token::{Token, TokenKind};

/// Literal represents a literal token.
pub struct Literal<'src>(Token<'src>);

impl<'src> Deref for Literal<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'src> AsRef<Token<'src>> for Literal<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl Display for Literal<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl<'src> TryFrom<Token<'src>> for Literal<'src> {
  type Error = super::Error<'src>;

  fn try_from(token: Token<'src>) -> super::Result<'src, Self> {
    match token.kind {
      TokenKind::Number
      | TokenKind::String
      | TokenKind::True
      | TokenKind::False
      | TokenKind::Nil => Ok(Literal(token)),
      _ => Err(super::Error::Literal(token)),
    }
  }
}
