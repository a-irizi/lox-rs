use crate::token::{Token, TokenKind};

use super::Scanner;

impl<'a> Scanner<'a> {
  /// Lex an identifier.
  ///
  /// an identifier maybe a keyword or a user definded identifier.
  pub(super) fn identifier(&mut self) -> Token<'a> {
    let lexeme = self
      .rest
      .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
      .map_or(self.rest, |index| &self.rest[..index]);

    self.rest = &self.rest[lexeme.len()..];

    match lexeme {
      "and" => self.just(TokenKind::And),
      "class" => self.just(TokenKind::Class),
      "else" => self.just(TokenKind::Else),
      "false" => self.just(TokenKind::False),
      "for" => self.just(TokenKind::For),
      "fun" => self.just(TokenKind::Fun),
      "if" => self.just(TokenKind::If),
      "nil" => self.just(TokenKind::Nil),
      "or" => self.just(TokenKind::Or),
      "print" => self.just(TokenKind::Print),
      "return" => self.just(TokenKind::Return),
      "super" => self.just(TokenKind::Super),
      "this" => self.just(TokenKind::This),
      "true" => self.just(TokenKind::True),
      "var" => self.just(TokenKind::Var),
      "while" => self.just(TokenKind::While),
      _ => self.with_lexeme(TokenKind::Identifier, lexeme),
    }
  }
}
