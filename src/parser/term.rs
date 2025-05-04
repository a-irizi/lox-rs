use super::macros::expr_or_return;
use crate::expr::{Expr, TermOperator, Terminal};
use crate::parser::Parser;
use crate::token::Token;

impl<'src, I> Parser<'src, I>
where
  I: Iterator<Item = Token<'src>>,
{
  /// parse a term expression.
  ///
  /// term has left-to-right associativity.
  pub(super) fn term(&mut self) -> Option<super::Result<'src>> {
    let mut expr = expr_or_return!(self.factor());

    while let Some(operator) = self
      .tokens
      .next_if(|t| TermOperator::matches(t))
      .map(|t| TryInto::<TermOperator>::try_into(t).expect("Term operators are binary operators"))
    {
      let right = expr_or_return!(self.factor());
      expr = Expr::binary(expr, operator.into(), right);
    }

    Some(Ok(expr))
  }
}
