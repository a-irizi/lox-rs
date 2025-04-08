#![allow(dead_code)]

mod block_comment;
mod error;
mod identifier;
mod line_comment;
mod number;
mod string;

use crate::token::{Token, TokenKind};

pub use self::error::{Error, Result};

pub struct Scanner<'a> {
  /// original input.
  source: &'a str,
  /// rest of input to scan, includes current character.
  rest: &'a str,
  /// line number reached.
  line: usize,
}

impl<'a> Scanner<'a> {
  pub fn new(source: &'a str) -> Self {
    Self { source, rest: source, line: 1 }
  }
}

impl<'a> Scanner<'a> {
  fn advance_line(&mut self) {
    if let Some(new_line_index) = self.rest.find('\n') {
      self.line += 1;
      self.rest = &self.rest[new_line_index + 1..];
    } else {
      self.rest = "";
    }
  }

  fn just(&self, kind: TokenKind) -> Token<'a> {
    Token::new(kind, None, self.line)
  }
  fn with_lexeme(&self, kind: TokenKind, lexeme: &'a str) -> Token<'a> {
    Token::new(kind, Some(lexeme), self.line)
  }
}

impl<'a> Scanner<'a> {
  fn single(&mut self, kind: TokenKind) -> Token<'a> {
    self.rest = &self.rest[1..];
    self.just(kind)
  }

  fn whitespace(&mut self, c: char) {
    if c == '\n' {
      self.line += 1;
    }
    self.rest = &self.rest[c.len_utf8()..];
  }
}

impl<'a> Iterator for Scanner<'a> {
  type Item = Result<Token<'a>>;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let mut chars = self.rest.chars();
      let c = chars.next()?;
      let c_rest = chars.as_str();

      let mut if_next_else = |c: char, yes, no| {
        let token = if c_rest.starts_with(c) {
          self.rest = &self.rest[c.len_utf8()..];
          yes
        } else {
          no
        };

        self.single(token)
      };

      let token = match c {
        // single character
        '(' => self.single(TokenKind::LeftParen),
        ')' => self.single(TokenKind::RightParen),
        '{' => self.single(TokenKind::LeftBrace),
        '}' => self.single(TokenKind::RightBrace),
        ',' => self.single(TokenKind::Comma),
        '.' => self.single(TokenKind::Dot),
        '-' => self.single(TokenKind::Minus),
        '+' => self.single(TokenKind::Plus),
        ';' => self.single(TokenKind::SemiColon),
        '*' => self.single(TokenKind::Star),

        // multi character
        '!' => if_next_else('=', TokenKind::BangEqual, TokenKind::Bang),
        '=' => if_next_else('=', TokenKind::EqualEqual, TokenKind::Equal),
        '<' => if_next_else('=', TokenKind::LessEqual, TokenKind::Less),
        '>' => if_next_else('=', TokenKind::GreaterEqual, TokenKind::Greater),

        '"' => match self.string() {
          Ok(token) => token,
          Err(e) => return Some(Err(e)),
        },

        // comments
        '/' => {
          if c_rest.starts_with('/') {
            self.line_comment();
            continue;
          } else if c_rest.starts_with('*') {
            match self.block_comment() {
              Ok(()) => continue,
              Err(e) => return Some(Err(e)),
            }
          }
          self.single(TokenKind::Slash)
        }
        // numbers
        '0'..='9' => self.number(),

        c if c.is_whitespace() => {
          self.whitespace(c);
          continue;
        }

        'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

        c => return Some(Err(Error::InvalidCharacter(c, self.line))),
      };

      return Some(Ok(token));
    }
  }
}

#[cfg(test)]
mod tests {
  use claims::{assert_matches, assert_none};

  use super::*;
  #[test]
  fn lex_empty_string() {
    let input = "";
    let mut scanner = Scanner::new(input);

    assert_none!(scanner.next());
  }
  #[test]
  fn lex_single_tokens_ok() {
    // arrange
    let input = "(){},.-+;";
    let mut scanner = Scanner::new(input);

    // act and assert
    let mut assert_just = |expected_kind| {
      assert_matches!(
        scanner.next(),
        Some(Ok(Token { kind, lexeme: None, line: 1 })) if kind == expected_kind,
        "Faild to match expected token {:?}", expected_kind
      );
    };
    assert_just(TokenKind::LeftParen);
    assert_just(TokenKind::RightParen);
    assert_just(TokenKind::LeftBrace);
    assert_just(TokenKind::RightBrace);
    assert_just(TokenKind::Comma);
    assert_just(TokenKind::Dot);
    assert_just(TokenKind::Minus);
    assert_just(TokenKind::Plus);
    assert_just(TokenKind::SemiColon);
    assert_none!(scanner.next());
  }

  #[test]
  fn lex_multi_character_ok() {
    let source = "< <= >= == <= !!=";
    let mut scanner = Scanner::new(source);

    let mut assert_just = |expected_kind| {
      assert_matches!(
        scanner.next(),
        Some(Ok(Token { kind, lexeme: None, line: 1 })) if kind == expected_kind,
        "Faild to match expected token {:?}", expected_kind
      );
    };

    assert_just(TokenKind::Less);
    assert_just(TokenKind::LessEqual);
    assert_just(TokenKind::GreaterEqual);
    assert_just(TokenKind::EqualEqual);
    assert_just(TokenKind::LessEqual);
    assert_just(TokenKind::Bang);
    assert_just(TokenKind::BangEqual);
    assert_none!(scanner.next());
  }

  #[test]
  fn lex_free_form_ok() {
    let source = "// this is a comment
(( )){} // grouping stuff
!*+-/=<> <= == // operators";
    let mut scanner = Scanner::new(source);
    let mut assert_just = |expected_kind, expected_line| {
      assert_matches!(
        scanner.next(),
        Some(Ok(Token { kind, lexeme: None, line })) if kind == expected_kind && line == expected_line,
        "Failed to match expected_kind {expected_kind:?}, expected_line {expected_line:?}"
      );
    };
    assert_just(TokenKind::LeftParen, 2);
    assert_just(TokenKind::LeftParen, 2);
    assert_just(TokenKind::RightParen, 2);
    assert_just(TokenKind::RightParen, 2);
    assert_just(TokenKind::LeftBrace, 2);
    assert_just(TokenKind::RightBrace, 2);
    assert_just(TokenKind::Bang, 3);
    assert_just(TokenKind::Star, 3);
    assert_just(TokenKind::Plus, 3);
    assert_just(TokenKind::Minus, 3);
    assert_just(TokenKind::Slash, 3);
    assert_just(TokenKind::Equal, 3);
    assert_just(TokenKind::Less, 3);
    assert_just(TokenKind::Greater, 3);
    assert_just(TokenKind::LessEqual, 3);
    assert_just(TokenKind::EqualEqual, 3);
    assert_none!(scanner.next());
  }
}
