use std::{fmt::Display, ops::Deref};

use crate::token::Token;

use super::{
  CommaOperator, Terminal, comparision_operator::ComparisonOperator,
  factor_operator::FactorOperator, term_operator::TermOperator,
};

/// Represents a binary operator token.
#[derive(Debug)]
pub struct BinaryOperator<'src>(Token<'src>);

impl<'src> Terminal<'src> for BinaryOperator<'src> {
  fn matches(token: &Token) -> bool {
    CommaOperator::matches(token)
      || ComparisonOperator::matches(token)
      || TermOperator::matches(token)
      || FactorOperator::matches(token)
  }
}

impl<'src> From<BinaryOperator<'src>> for Token<'src> {
  fn from(value: BinaryOperator<'src>) -> Self {
    value.0
  }
}

impl<'src> AsRef<Token<'src>> for BinaryOperator<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl<'src> Deref for BinaryOperator<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'src> TryFrom<Token<'src>> for BinaryOperator<'src> {
  type Error = crate::expr::Error<'src>;

  fn try_from(token: Token<'src>) -> Result<Self, Self::Error> {
    if <Self as Terminal>::matches(&token) {
      Ok(BinaryOperator(token))
    } else {
      Err(Self::Error::BinaryOperator(token))
    }
  }
}

impl<'src> From<ComparisonOperator<'src>> for BinaryOperator<'src> {
  fn from(operator: ComparisonOperator<'src>) -> Self {
    BinaryOperator(operator.into_token())
  }
}

impl<'src> From<TermOperator<'src>> for BinaryOperator<'src> {
  fn from(operator: TermOperator<'src>) -> Self {
    BinaryOperator(operator.into_token())
  }
}

impl<'src> From<FactorOperator<'src>> for BinaryOperator<'src> {
  fn from(operator: FactorOperator<'src>) -> Self {
    BinaryOperator(operator.into_token())
  }
}

impl Display for BinaryOperator<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
