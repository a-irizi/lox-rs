use super::macros::expr_or_return;
use crate::expr::{Expr, FactorOperator, Terminal};
use crate::parser::Parser;
use crate::token::Token;

impl<'src, I> Parser<'src, I>
where
  I: Iterator<Item = Token<'src>>,
{
  /// parse a factor expression.
  ///
  /// factor has left-to-right associativity.
  pub(super) fn factor(&mut self) -> Option<super::Result<'src>> {
    let mut expr = expr_or_return!(self.unary());

    while let Some(operator) = self
      .tokens
      .next_if(|t| FactorOperator::matches(t))
      .map(|t| TryInto::<FactorOperator>::try_into(t).expect("Term operators are binary operators"))
    {
      let right = expr_or_return!(self.unary());
      expr = Expr::binary(expr, operator.into(), right);
    }

    Some(Ok(expr))
  }
}
