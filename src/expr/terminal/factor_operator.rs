use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct FactorOperator<'src>(Token<'src>);

impl<'src> Terminal<'src> for FactorOperator<'src> {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Star | TokenKind::Slash)
  }
}

impl<'src> From<FactorOperator<'src>> for Token<'src> {
  fn from(value: FactorOperator<'src>) -> Self {
    value.0
  }
}

impl<'src> AsRef<Token<'src>> for FactorOperator<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl<'src> Deref for FactorOperator<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'src> TryFrom<Token<'src>> for FactorOperator<'src> {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <FactorOperator as Terminal>::matches(&token) {
      Ok(FactorOperator(token))
    } else {
      Err(expr::Error::FactorOperator(token))
    }
  }
}

impl Display for FactorOperator<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
