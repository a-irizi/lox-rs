use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct TernaryElseOperator(TokenKind);

impl Terminal for TernaryElseOperator {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Colon)
  }
}

impl From<TernaryElseOperator> for TokenKind {
  fn from(value: TernaryElseOperator) -> Self {
    value.0
  }
}

impl<'src> TryFrom<Token<'src>> for TernaryElseOperator {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <Self as Terminal>::matches(&token) {
      Ok(TernaryElseOperator(token.kind))
    } else {
      Err(expr::Error::TernaryElseOperator(token))
    }
  }
}

impl AsRef<TokenKind> for TernaryElseOperator {
  fn as_ref(&self) -> &TokenKind {
    &self.0
  }
}

impl Deref for TernaryElseOperator {
  type Target = TokenKind;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for TernaryElseOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
