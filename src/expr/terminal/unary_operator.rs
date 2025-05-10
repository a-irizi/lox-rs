use std::{fmt::Display, ops::Deref};

use crate::{
  expr::error,
  token::{Token, TokenKind},
};

use super::Terminal;

/// represents a unary operator token.
#[derive(Debug)]
pub struct UnaryOperator(TokenKind);

impl Terminal for UnaryOperator {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Bang | TokenKind::Minus)
  }
}

impl From<UnaryOperator> for TokenKind {
  fn from(value: UnaryOperator) -> Self {
    value.0
  }
}

impl AsRef<TokenKind> for UnaryOperator {
  fn as_ref(&self) -> &TokenKind {
    &self.0
  }
}

impl Deref for UnaryOperator {
  type Target = TokenKind;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for UnaryOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl<'src> TryFrom<Token<'src>> for UnaryOperator {
  type Error = error::Error<'src>;

  fn try_from(token: Token<'src>) -> error::Result<'src, Self> {
    if <Self as Terminal>::matches(&token) {
      Ok(UnaryOperator(token.kind))
    } else {
      Err(error::Error::UnaryOperator(token))
    }
  }
}
