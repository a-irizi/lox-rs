use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct TermOperator<'src>(Token<'src>);

impl<'src> Terminal<'src> for TermOperator<'src> {
  fn matches(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Plus | TokenKind::Minus)
  }
}

impl<'src> From<TermOperator<'src>> for Token<'src> {
  fn from(value: TermOperator<'src>) -> Self {
    value.0
  }
}

impl<'src> AsRef<Token<'src>> for TermOperator<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl<'src> Deref for TermOperator<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'src> TryFrom<Token<'src>> for TermOperator<'src> {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <TermOperator as Terminal>::matches(&token) {
      Ok(TermOperator(token))
    } else {
      Err(expr::Error::TermOperator(token))
    }
  }
}

impl Display for TermOperator<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
