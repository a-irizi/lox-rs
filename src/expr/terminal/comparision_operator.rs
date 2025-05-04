use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct ComparisonOperator<'src>(Token<'src>);

impl<'src> Terminal<'src> for ComparisonOperator<'src> {
  fn matches(token: &Token) -> bool {
    matches!(
      token.kind,
      TokenKind::BangEqual
        | TokenKind::EqualEqual
        | TokenKind::Less
        | TokenKind::LessEqual
        | TokenKind::Greater
        | TokenKind::GreaterEqual
    )
  }
}

impl<'src> From<ComparisonOperator<'src>> for Token<'src> {
  fn from(value: ComparisonOperator<'src>) -> Self {
    value.0
  }
}

impl<'src> AsRef<Token<'src>> for ComparisonOperator<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl<'src> Deref for ComparisonOperator<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'src> TryFrom<Token<'src>> for ComparisonOperator<'src> {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <ComparisonOperator as Terminal>::matches(&token) {
      Ok(ComparisonOperator(token))
    } else {
      Err(expr::Error::ComparisonOperator(token))
    }
  }
}

impl Display for ComparisonOperator<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
