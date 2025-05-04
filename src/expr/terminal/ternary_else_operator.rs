use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct TernaryElseOperator<'src>(Token<'src>);

impl<'src> Terminal<'src> for TernaryElseOperator<'src> {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Colon)
  }
}

impl<'src> From<TernaryElseOperator<'src>> for Token<'src> {
  fn from(value: TernaryElseOperator<'src>) -> Self {
    value.0
  }
}

impl<'src> TryFrom<Token<'src>> for TernaryElseOperator<'src> {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <Self as Terminal>::matches(&token) {
      Ok(TernaryElseOperator(token))
    } else {
      Err(expr::Error::TernaryElseOperator(token))
    }
  }
}

impl<'src> AsRef<Token<'src>> for TernaryElseOperator<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl<'src> Deref for TernaryElseOperator<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for TernaryElseOperator<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
