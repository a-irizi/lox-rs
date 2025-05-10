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
          Some(Token::new(
            TokenKind::String(lexeme[1..lexeme.len() - 1].into()),
            Some(lexeme),
            self.line,
          ))
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
  use claims::{assert_matches, assert_none, assert_ok, assert_some};

  use super::*;

  #[test]
  fn lex_string_one_line_ok() {
    let literal = "this is a string literal";
    let source = format!("{literal:?}");
    let mut scanner = Scanner::new(&source);
    let token = assert_some!(scanner.next());
    let token = assert_ok!(token);

    assert_matches!(
      token,
      Token { kind: TokenKind::String(s), lexeme: Some(l), line: 1 } if s == literal && l == format!(r#""{literal}""#)
    );
    assert_none!(scanner.next());
  }

  #[test]
  fn lex_string_multi_line_ok() {
    let literal = "this is a string literal
string literal can cross multiple lines.
just like rust's.";
    let source = format!(
      r#""{literal}"
  "#,
    );
    let mut scanner = Scanner::new(&source);

    let token = scanner.next();
    let token = assert_some!(token);
    let token = assert_ok!(token);

    assert_matches!(
      token,
      Token {
        kind: TokenKind::String(s),
        line: 3,
        lexeme: Some( l )
      } if s == literal && l == format!(r#""{literal}""#)
    );
    assert_none!(scanner.next());
  }
}
