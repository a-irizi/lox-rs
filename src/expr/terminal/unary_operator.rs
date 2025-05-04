use std::{fmt::Display, ops::Deref};

use crate::{
  expr::error,
  token::{Token, TokenKind},
};

use super::Terminal;

/// represents a unary operator token.
#[derive(Debug)]
pub struct UnaryOperator<'src>(Token<'src>);

impl<'src> Terminal<'src> for UnaryOperator<'src> {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Bang | TokenKind::Minus)
  }
}

impl<'src> From<UnaryOperator<'src>> for Token<'src> {
  fn from(value: UnaryOperator<'src>) -> Self {
    value.0
  }
}

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
    if <Self as Terminal>::matches(&token) {
      Ok(UnaryOperator(token))
    } else {
      Err(error::Error::UnaryOperator(token))
    }
  }
}
