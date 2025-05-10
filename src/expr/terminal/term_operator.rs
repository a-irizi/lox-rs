use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct TermOperator(TokenKind);

impl Terminal for TermOperator {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Plus | TokenKind::Minus)
  }
}

impl From<TermOperator> for TokenKind {
  fn from(value: TermOperator) -> Self {
    value.0
  }
}

impl AsRef<TokenKind> for TermOperator {
  fn as_ref(&self) -> &TokenKind {
    &self.0
  }
}

impl Deref for TermOperator {
  type Target = TokenKind;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'src> TryFrom<Token<'src>> for TermOperator {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <TermOperator as Terminal>::matches(&token) {
      Ok(TermOperator(token.kind))
    } else {
      Err(expr::Error::TermOperator(token))
    }
  }
}

impl Display for TermOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
