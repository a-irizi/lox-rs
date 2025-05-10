use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct TernaryThenOperator(TokenKind);

impl Terminal for TernaryThenOperator {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Question)
  }
}

impl From<TernaryThenOperator> for TokenKind {
  fn from(value: TernaryThenOperator) -> Self {
    value.0
  }
}

impl<'src> TryFrom<Token<'src>> for TernaryThenOperator {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <Self as Terminal>::matches(&token) {
      Ok(TernaryThenOperator(token.kind))
    } else {
      Err(expr::Error::TernaryThenOperator(token))
    }
  }
}

impl AsRef<TokenKind> for TernaryThenOperator {
  fn as_ref(&self) -> &TokenKind {
    &self.0
  }
}

impl Deref for TernaryThenOperator {
  type Target = TokenKind;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for TernaryThenOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
