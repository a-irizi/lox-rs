mod binary_operator;
mod comma_operator;
mod comparision_operator;
mod factor_operator;
mod literal;
mod term_operator;
mod ternary_else_operator;
mod ternary_then_operator;
mod unary_operator;

use std::{
  fmt::{Debug, Display},
  ops::Deref,
};

use crate::token::{Token, TokenKind};

pub use self::{
  binary_operator::BinaryOperator, comma_operator::CommaOperator,
  comparision_operator::ComparisonOperator, factor_operator::FactorOperator, literal::Literal,
  term_operator::TermOperator, ternary_else_operator::TernaryElseOperator,
  ternary_then_operator::TernaryThenOperator, unary_operator::UnaryOperator,
};

pub trait Terminal:
  for<'src> TryFrom<Token<'src>>
  + Into<TokenKind>
  + AsRef<TokenKind>
  + Deref<Target = TokenKind>
  + Debug
  + Display
{
  fn matches(token: &Token) -> bool;
}
