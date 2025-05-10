use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct CommaOperator(TokenKind);

impl Terminal for CommaOperator {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Comma)
  }
}

impl From<CommaOperator> for TokenKind {
  fn from(value: CommaOperator) -> Self {
    value.0
  }
}

impl<'src> TryFrom<Token<'src>> for CommaOperator {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <Self as Terminal>::matches(&token) {
      Ok(CommaOperator(token.kind))
    } else {
      Err(expr::Error::CommaOperator(token))
    }
  }
}

impl AsRef<TokenKind> for CommaOperator {
  fn as_ref(&self) -> &TokenKind {
    &self.0
  }
}

impl Deref for CommaOperator {
  type Target = TokenKind;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for CommaOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
