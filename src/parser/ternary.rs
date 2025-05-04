use crate::{
  expr::{Expr, Terminal, TernaryElseOperator, TernaryThenOperator},
  token::Token,
};

use super::{Parser, macros::expr_or_return};

impl<'src, I> Parser<'src, I>
where
  I: Iterator<Item = Token<'src>>,
{
  /// parse a ternary expression.
  ///
  /// ternary has right-to-left associativity.
  pub(super) fn ternary(&mut self) -> Option<super::Result<'src>> {
    let mut expr = expr_or_return!(self.comparison());

    if let Some(then_operator) = self.tokens.next_if(|t| TernaryThenOperator::matches(t)) {
      let then_branch = match self.expression() {
        Some(Ok(expr)) => expr,
        None => {
          return Some(Err(super::Error::TernaryMissingThenBranch {
            question: expr,
            then_operator,
          }));
        }
        err => return err,
      };

      let Some(else_operator) = self.tokens.next_if(|t| TernaryElseOperator::matches(t)) else {
        return Some(Err(super::Error::TernaryMissingElseOperator {
          question: expr,
          then_operator,
          then_branch,
        }));
      };
      let else_branch = match self.ternary() {
        Some(Ok(else_branch)) => else_branch,
        None => {
          return Some(Err(super::Error::TernaryMissingElseBranch {
            question: expr,
            then_operator,
            then_branch,
            else_operator,
          }));
        }
        err => return err,
      };
      expr = Expr::ternary(expr, then_branch, else_branch);
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
  fn parser_ternary_single_ok() {
    let src = "1 > 2 ? 2> 3 : 5 < 6";
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
    let result = parser.ternary();
    let result = assert_some!(result);
    let expr = assert_ok!(result);
    assert_matches!(expr, Expr::Ternary { condition: _, then_branch: _, else_branch: _ });
    println!("{:?}", AstFormatter.visit(&expr));
  }

  #[test]
  fn parser_ternary_multiple_ok() {
    let src = "true ? 43 : false ? 1 <= 2 ? 10 : 123 : 224";
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
    let result = parser.ternary();
    let result = assert_some!(result);
    let expr = assert_ok!(result);
    assert_matches!(expr, Expr::Ternary { condition: _, then_branch: _, else_branch: _ });
    println!("{:?}", AstFormatter.visit(&expr));
  }
}
