use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct CommaOperator<'src>(Token<'src>);

impl<'src> Terminal<'src> for CommaOperator<'src> {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Comma)
  }
}

impl<'src> From<CommaOperator<'src>> for Token<'src> {
  fn from(value: CommaOperator<'src>) -> Self {
    value.0
  }
}

impl<'src> TryFrom<Token<'src>> for CommaOperator<'src> {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <Self as Terminal>::matches(&token) {
      Ok(CommaOperator(token))
    } else {
      Err(expr::Error::CommaOperator(token))
    }
  }
}

impl<'src> AsRef<Token<'src>> for CommaOperator<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl<'src> Deref for CommaOperator<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for CommaOperator<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
