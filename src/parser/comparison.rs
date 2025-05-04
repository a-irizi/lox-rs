use super::macros::expr_or_return;
use crate::expr::{ComparisonOperator, Expr, Terminal};
use crate::parser::Parser;
use crate::token::Token;

impl<'src, I> Parser<'src, I>
where
  I: Iterator<Item = Token<'src>>,
{
  /// Parses a comparison expression.
  ///
  /// comparison has left-to-right associativity.
  pub(super) fn comparison(&mut self) -> Option<super::Result<'src>> {
    let mut expr = expr_or_return!(self.term());

    while let Some(operator) = self.tokens.next_if(|t| ComparisonOperator::matches(t)).map(|t| {
      TryInto::<ComparisonOperator>::try_into(t).expect("comparison operators are binary operators")
    }) {
      let right = expr_or_return!(self.term());
      expr = Expr::binary(expr, operator.into(), right);
    }

    Some(Ok(expr))
  }
}

#[cfg(test)]
mod tests {
  use claims::{assert_matches, assert_ok, assert_some};

  use crate::{
    expr::{AstFormatter, ExprVisitor},
    scanner::Scanner,
  };

  use super::*;

  #[test]
  fn parser_comparison_ok() {
    let src = "43 < 52";
    let mut had_errors = false;
    let tokens = Scanner::new(src)
      .inspect(|t| {
        if t.is_err() {
          had_errors = true;
        }
      })
      .collect::<Vec<_>>();
    assert!(!had_errors);
    let tokens = tokens.into_iter().map(|t| t.unwrap());
    let mut parser = Parser::new(tokens);
    let result = parser.comparison();
    let result = assert_some!(result);
    let expr = assert_ok!(result);
    assert_matches!(expr, Expr::Binary { left: _, operator: _, right: _ });
    println!("{:?}", AstFormatter.visit(&expr));
  }
}
