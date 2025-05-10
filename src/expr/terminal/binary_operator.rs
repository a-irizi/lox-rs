use std::{fmt::Display, ops::Deref};

use crate::token::{Token, TokenKind};

use super::{
  CommaOperator, Terminal, comparision_operator::ComparisonOperator,
  factor_operator::FactorOperator, term_operator::TermOperator,
};

/// Represents a binary operator token.
#[derive(Debug)]
pub struct BinaryOperator(TokenKind);

impl Terminal for BinaryOperator {
  fn matches(token: &Token) -> bool {
    CommaOperator::matches(token)
      || ComparisonOperator::matches(token)
      || TermOperator::matches(token)
      || FactorOperator::matches(token)
  }
}

impl From<BinaryOperator> for TokenKind {
  fn from(value: BinaryOperator) -> Self {
    value.0
  }
}

impl AsRef<TokenKind> for BinaryOperator {
  fn as_ref(&self) -> &TokenKind {
    &self.0
  }
}

impl Deref for BinaryOperator {
  type Target = TokenKind;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'src> TryFrom<Token<'src>> for BinaryOperator {
  type Error = crate::expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <Self as Terminal>::matches(&token) {
      Ok(BinaryOperator(token.kind))
    } else {
      Err(Self::Error::BinaryOperator(token))
    }
  }
}

impl From<ComparisonOperator> for BinaryOperator {
  fn from(operator: ComparisonOperator) -> Self {
    BinaryOperator(operator.into())
  }
}

impl From<TermOperator> for BinaryOperator {
  fn from(operator: TermOperator) -> Self {
    BinaryOperator(operator.into())
  }
}

impl From<FactorOperator> for BinaryOperator {
  fn from(operator: FactorOperator) -> Self {
    BinaryOperator(operator.into())
  }
}

impl Display for BinaryOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
