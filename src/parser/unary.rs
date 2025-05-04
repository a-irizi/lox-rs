use crate::expr::{Expr, Terminal, UnaryOperator};
use crate::parser::Parser;
use crate::parser::macros::expr_or_return;
use crate::token::Token;

impl<'src, I> Parser<'src, I>
where
  I: Iterator<Item = Token<'src>>,
{
  /// parse a unary expression.
  ///
  /// unary has left-to-right associativity.
  pub(super) fn unary(&mut self) -> Option<super::Result<'src>> {
    let Some(operator) =
      self.tokens.next_if(|t| UnaryOperator::matches(t)).map(|t| t.try_into().unwrap())
    else {
      return self.primary();
    };

    let right = expr_or_return!(self.primary());
    let expr = Expr::unary(operator, right);

    Some(Ok(expr))
  }
}
