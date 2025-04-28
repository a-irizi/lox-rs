use crate::token::Token;

pub struct Parser<'src, T>
where
  T: Iterator<Item = Token<'src>>,
{
  tokens: T,
}

impl<'src, T> Parser<'src, T>
where
  T: Iterator<Item = Token<'src>>,
{
  pub fn new(tokens: T) -> Self {
    Self { tokens }
  }
}
