use std::{fmt::Display, ops::Deref};

use crate::token::{Token, TokenKind};

pub struct LiteralToken<'src>(Token<'src>);

impl<'src> Deref for LiteralToken<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'src> AsRef<Token<'src>> for LiteralToken<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl Display for LiteralToken<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl<'src> TryFrom<Token<'src>> for LiteralToken<'src> {
  type Error = String;

  fn try_from(value: Token<'src>) -> Result<Self, Self::Error> {
    match value.kind {
      TokenKind::Number
      | TokenKind::String
      | TokenKind::True
      | TokenKind::False
      | TokenKind::Nil => Ok(LiteralToken(value)),
      _ => Err(format!("Invalid literal token {value:?}")),
    }
  }
}
