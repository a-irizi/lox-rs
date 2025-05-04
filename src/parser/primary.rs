use crate::expr::{Expr, Literal, Terminal};
use crate::parser::Parser;
use crate::token::{Token, TokenKind};

impl<'src, I> Parser<'src, I>
where
  I: Iterator<Item = Token<'src>>,
{
  pub(super) fn primary(&mut self) -> Option<super::Result<'src>> {
    if let Some(literal) =
      self.tokens.next_if(|t| Literal::matches(t)).map(|t| Expr::literal(t.try_into().unwrap()))
    {
      return Some(Ok(literal));
    }

    if let Some(right_paren) = self.tokens.next_if(|t| t.kind == TokenKind::LeftParen) {
      let expr = match self.expression() {
        Some(Ok(expr)) => expr,
        None => return Some(Err(super::Error::GroupingMissingExpression { right_paren })),
        err => return err,
      };

      return match self.tokens.next_if(|t| t.kind == TokenKind::RightParen) {
        Some(_) => Some(Ok(Expr::grouping(expr))),
        None => Some(Err(super::Error::GroupingNotClosed { right_paren, expr })),
      };
    }

    None
  }
}
