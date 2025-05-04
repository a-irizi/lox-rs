use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct TernaryThenOperator<'src>(Token<'src>);

impl<'src> Terminal<'src> for TernaryThenOperator<'src> {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Question)
  }
}

impl<'src> From<TernaryThenOperator<'src>> for Token<'src> {
  fn from(value: TernaryThenOperator<'src>) -> Self {
    value.0
  }
}

impl<'src> TryFrom<Token<'src>> for TernaryThenOperator<'src> {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <Self as Terminal>::matches(&token) {
      Ok(TernaryThenOperator(token))
    } else {
      Err(expr::Error::TernaryThenOperator(token))
    }
  }
}

impl<'src> AsRef<Token<'src>> for TernaryThenOperator<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl<'src> Deref for TernaryThenOperator<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for TernaryThenOperator<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
