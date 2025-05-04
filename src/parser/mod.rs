mod comma;
mod comparison;
mod error;
mod expression;
mod factor;
mod macros;
mod primary;
mod term;
mod ternary;
mod unary;

use std::iter::Peekable;

use self::error::{Error, Result};
use crate::token::Token;

pub struct Parser<'src, I>
where
  I: Iterator<Item = Token<'src>>,
{
  tokens: Peekable<I>,
}

impl<'src, I> Parser<'src, I>
where
  I: Iterator<Item = Token<'src>>,
{
  pub fn new(tokens: I) -> Self {
    Self { tokens: tokens.peekable() }
  }
}

impl<'src, T> Iterator for Parser<'src, T>
where
  T: Iterator<Item = Token<'src>>,
{
  type Item = self::Result<'src>;

  fn next(&mut self) -> Option<Self::Item> {
    self.expression()
  }
}

#[cfg(test)]
mod tests {
  use claims::{assert_matches, assert_ok, assert_some};

  use super::*;
  use crate::expr::{AstFormatter, Expr, ExprVisitor};
  use crate::scanner::Scanner;
  use std::time::Instant;

  #[test]
  fn parser_simple_ok() {
    let src = r#"(
    4 + 5 * 3 / 7 <= - 2 + (-1) + "anass" ? 1, 2, 5 + 5 ? 7 : 8 : 9
)"#;
    let mut had_error = false;
    let now = Instant::now();
    let tokens: Vec<_> = Scanner::new(src)
      .inspect(|t| {
        if t.is_err() {
          had_error = true;
        }
      })
      .map_while(std::result::Result::ok)
      .collect();
    assert!(!had_error, "Failed to parse source code");
    let mut parser = Parser::new(tokens.into_iter());
    let expr = parser.next();
    print!("took {:?} ns", now.elapsed().as_micros());
    let expr = assert_some!(expr);
    let expr = assert_ok!(expr);
    assert_matches!(expr, Expr::Grouping(_));
    println!("{}", AstFormatter.visit(&expr));
  }
}
