use crate::{
  expr::{CommaOperator, Expr, Terminal},
  parser::macros::expr_or_return,
  token::Token,
};

use super::Parser;

impl<'src, I> Parser<'src, I>
where
  I: Iterator<Item = Token<'src>>,
{
  /// Parses a comma expression.
  ///
  /// comma expressions have the lowest precedence with left-to-right associativity.
  pub(super) fn comma(&mut self) -> Option<super::Result<'src>> {
    let mut expr = expr_or_return!(self.ternary());

    while let Some(operator) =
      self.tokens.next_if(|t| CommaOperator::matches(t)).map(|t| t.try_into().unwrap())
    {
      let right = expr_or_return!(self.ternary());
      expr = Expr::binary(expr, operator, right);
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
  fn parser_comma_ok() {
    let src = "43, 52";
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
    let result = parser.comma();
    let result = assert_some!(result);
    let expr = assert_ok!(result);
    assert_matches!(expr, Expr::Binary { left: _, operator: _, right: _ });
    println!("{:?}", AstFormatter.visit(&expr));
  }

  #[test]
  fn parser_comma_multiple_ok() {
    let src = "43, 52, 123";
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
    let result = parser.comma();
    let result = assert_some!(result);
    let expr = assert_ok!(result);
    assert_matches!(expr, Expr::Binary { left: _, operator: _, right: _ });
    println!("{:?}", AstFormatter.visit(&expr));
  }
}
