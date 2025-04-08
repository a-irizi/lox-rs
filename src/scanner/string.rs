use crate::token::{Token, TokenKind};

use super::{Error, Result, Scanner};

impl<'a> Scanner<'a> {
  /// Lex a string literal.
  pub(super) fn string(&mut self) -> Result<Token<'a>> {
    self
      .rest
      .char_indices()
      // skip starting "
      .skip(1)
      .find_map(|(index, c)| match c {
        '"' => {
          let lexeme = &self.rest[..=index];
          self.rest = &self.rest[index + 1..];
          Some(Token::new(TokenKind::String, Some(lexeme), self.line))
        }
        c => {
          if c == '\n' {
            self.line += 1;
          }
          None
        }
      })
      .ok_or_else(|| {
        self.rest = "";
        Error::UnterminatedString
      })
  }
}

#[cfg(test)]
mod tests {
  use claims::{assert_matches, assert_none};

  use super::*;

  #[test]
  fn lex_string_one_line_ok() {
    let source = r#""this is a string literal""#;
    let mut scanner = Scanner::new(source);

    assert_matches!(
      scanner.next(),
      Some(Ok(Token {
        kind: TokenKind::String,
        lexeme: Some(r#""this is a string literal""#),
        line: 1
      }))
    );
    assert_none!(scanner.next());
  }

  #[test]
  fn lex_string_multi_line_ok() {
    let source = r#""this is a string literal
string literal can cross multiple lines.
just like rust's."
  "#;
    let mut scanner = Scanner::new(source);

    assert_matches!(
      scanner.next(),
      Some(Ok(Token {
        kind: TokenKind::String,
        line: 3,
        lexeme: Some(
          r#""this is a string literal
string literal can cross multiple lines.
just like rust's.""#
        )
      }))
    );
    assert_none!(scanner.next());
  }
}
