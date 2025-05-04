use std::{fmt::Display, ops::Deref};

use crate::expr;
use crate::token::{Token, TokenKind};

use super::Terminal;
/// Literal represents a literal token.
#[derive(Debug)]
pub struct Literal<'src>(Token<'src>);

impl<'src> Terminal<'src> for Literal<'src> {
  fn matches(token: &Token) -> bool {
    matches!(
      token.kind,
      TokenKind::Number | TokenKind::String | TokenKind::True | TokenKind::False | TokenKind::Nil
    )
  }
}

impl<'src> From<Literal<'src>> for Token<'src> {
  fn from(value: Literal<'src>) -> Self {
    value.0
  }
}

impl<'src> Deref for Literal<'src> {
  type Target = Token<'src>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'src> AsRef<Token<'src>> for Literal<'src> {
  fn as_ref(&self) -> &Token<'src> {
    &self.0
  }
}

impl Display for Literal<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl<'src> TryFrom<Token<'src>> for Literal<'src> {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> expr::Result<'src, Self> {
    if <Self as Terminal>::matches(&token) {
      Ok(Literal(token))
    } else {
      Err(expr::Error::Literal(token))
    }
  }
}
