use std::{fmt::Display, ops::Deref};

use crate::{
  expr,
  token::{Token, TokenKind},
};

use super::Terminal;

#[derive(Debug)]
pub struct ComparisonOperator(TokenKind);

impl Terminal for ComparisonOperator {
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

impl From<ComparisonOperator> for TokenKind {
  fn from(value: ComparisonOperator) -> Self {
    value.0
  }
}

impl AsRef<TokenKind> for ComparisonOperator {
  fn as_ref(&self) -> &TokenKind {
    &self.0
  }
}

impl Deref for ComparisonOperator {
  type Target = TokenKind;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'src> TryFrom<Token<'src>> for ComparisonOperator {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <ComparisonOperator as Terminal>::matches(&token) {
      Ok(ComparisonOperator(token.kind))
    } else {
      Err(expr::Error::ComparisonOperator(token))
    }
  }
}

impl Display for ComparisonOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
