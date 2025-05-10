use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct FactorOperator(TokenKind);

impl Terminal for FactorOperator {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Star | TokenKind::Slash)
  }
}

impl From<FactorOperator> for TokenKind {
  fn from(value: FactorOperator) -> Self {
    value.0
  }
}

impl AsRef<TokenKind> for FactorOperator {
  fn as_ref(&self) -> &TokenKind {
    &self.0
  }
}

impl Deref for FactorOperator {
  type Target = TokenKind;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'src> TryFrom<Token<'src>> for FactorOperator {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <FactorOperator as Terminal>::matches(&token) {
      Ok(FactorOperator(token.kind))
    } else {
      Err(expr::Error::FactorOperator(token))
    }
  }
}

impl Display for FactorOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
