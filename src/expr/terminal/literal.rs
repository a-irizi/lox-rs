use std::{fmt::Display, ops::Deref};

use crate::expr;
use crate::token::{Token, TokenKind};

use super::Terminal;
/// Literal represents a literal token.
#[derive(Debug)]
pub struct Literal(TokenKind);

impl Terminal for Literal {
  fn matches(token: &Token) -> bool {
    matches!(
      token.kind,
      TokenKind::Number(_)
        | TokenKind::String(_)
        | TokenKind::True
        | TokenKind::False
        | TokenKind::Nil
    )
  }
}

impl From<Literal> for TokenKind {
  fn from(value: Literal) -> Self {
    value.0
  }
}

impl Deref for Literal {
  type Target = TokenKind;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl AsRef<TokenKind> for Literal {
  fn as_ref(&self) -> &TokenKind {
    &self.0
  }
}

impl Display for Literal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl<'src> TryFrom<Token<'src>> for Literal {
  type Error = expr::Error<'src>;

  fn try_from(token: Token<'src>) -> expr::Result<'src, Self> {
    if <Self as Terminal>::matches(&token) {
      Ok(Literal(token.kind))
    } else {
      Err(expr::Error::Literal(token))
    }
  }
}
