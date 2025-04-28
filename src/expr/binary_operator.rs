use std::{fmt::Display, ops::Deref};

use crate::token::{Token, TokenKind};

/// Represents a binary operator token.
pub struct BinaryOperator<'src>(Token<'src>);

impl<'src> AsRef<Token<'src>> for BinaryOperator<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl<'src> Deref for BinaryOperator<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for BinaryOperator<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl<'src> TryFrom<Token<'src>> for BinaryOperator<'src> {
  type Error = super::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    match token.kind {
      TokenKind::BangEqual
      | TokenKind::EqualEqual
      | TokenKind::Less
      | TokenKind::LessEqual
      | TokenKind::Greater
      | TokenKind::GreaterEqual
      | TokenKind::Plus
      | TokenKind::Minus
      | TokenKind::Star
      | TokenKind::Slash => Ok(BinaryOperator(token)),
      _ => Err(super::Error::BinaryOperator(token)),
    }
  }
}
