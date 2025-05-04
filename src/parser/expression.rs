use crate::parser::Parser;
use crate::token::Token;

impl<'src, I> Parser<'src, I>
where
  I: Iterator<Item = Token<'src>>,
{
  /// parse a general expression.
  pub(super) fn expression(&mut self) -> Option<super::Result<'src>> {
    self.comma()
  }
}
