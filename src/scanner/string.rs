use crate::token::{Token, TokenKind};

use super::{Error, Result, Scanner};

impl<'src> Scanner<'src> {
  /// Lex a string literal.
  pub(super) fn string(&mut self) -> Result<'src, Token<'src>> {
    let first_quote = &self.rest[..1];
    let line = self.line;
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
        Error::UnterminatedString { start: first_quote, line }
      })
  }
}

#[cfg(test)]
mod tests {
  use claims::{assert_err, assert_none, assert_ok, assert_some};
  use proptest::{prelude::*, prop_compose, proptest};
  use rstest::rstest;

  use super::*;

  #[rstest(literal)]
  #[case::empty("")]
  #[case::one_line("this is a string literal")]
  #[case::multi_line(
    "this is a string literal
string literal can cross multiple lines.
just like rust's."
  )]
  #[case::escaped_new_line("\n")]
  #[case::escaped_null_character("\0")]
  fn lex_string_ok(literal: &str) {
    let line = literal.chars().filter(|&c| c == '\n').count();
    let source = format!(
      r#""{literal}"
  "#,
    );
    let mut scanner = Scanner::new(&source);

    let token = scanner.next();
    let token = assert_some!(token);
    let token = assert_ok!(token);

    match token {
      Token { kind: TokenKind::String(s), line: line2, lexeme: Some(lexeme) } => {
        assert_eq!(s, literal);
        assert_eq!(line + 1, line2);
        assert_eq!(lexeme, format!(r#""{literal}""#));
      }
      _ => panic!("Wrong token: {token:?}"),
    }

    assert_none!(scanner.next());
  }

  #[rstest(literal)]
  #[case::empty("")]
  #[case::one_line("this is a string literal")]
  #[case::multi_line(
    "this is a string literal
  string literal can cross multiple lines.
  just like rust's."
  )]
  #[case::escaped_new_line("\n")]
  #[case::escaped_null_character("\0")]
  fn lex_string_err(literal: &str) {
    let source = format!(
      r#""{literal}
        "#,
    );
    let mut scanner = Scanner::new(&source);

    let next = scanner.next();
    let next = assert_some!(next);
    let error = assert_err!(next);

    match error {
      Error::UnterminatedString { start, line } => {
        let start2 = &source[..1];
        assert_eq!(start, start2);
        assert_eq!(start.as_ptr(), start2.as_ptr());
        assert_eq!(line, 1);
      }
      _ => panic!("Wrong error: {error:?}"),
    }

    assert_none!(scanner.next());
  }

  prop_compose! {
    fn arb_string_literal()(s in r#"[^"]"#) -> String {
      s.replace('\\', r"\\")
    }
  }

  proptest! {
    #[test]
    fn lex_arb_string_ok(literal in arb_string_literal()) {
      println!("literal {literal:?}");
      let line = literal.chars().filter(|&c| c == '\n').count();
      let source = format!(
        r#""{literal}"
    "#,
      );
      let mut scanner = Scanner::new(&source);

      let token = scanner.next();
      let token = assert_some!(token);
      let token = assert_ok!(token);

      match token {
        Token { kind: TokenKind::String(s), line: line2, lexeme: Some(lexeme) } => {
          prop_assert_eq!(s, literal.clone());
          prop_assert_eq!(line+1, line2);
          prop_assert_eq!(lexeme, format!(r#""{literal}""#));
        }
        _ => panic!("Wrong token: {token:?}")
        }

      assert_none!(scanner.next());
    }
  }

  proptest! {
  #[test]
  fn lex_arb_string_err(literal in arb_string_literal()) {
      let source = format!(
        r#""{literal}
          "#,
      );
      let mut scanner = Scanner::new(&source);

      let next = scanner.next();
      let next = assert_some!(next);
      let error = assert_err!(next);

      match error {
        Error::UnterminatedString { start, line } => {
          let start2 = &source[..1];
          assert_eq!(start, start2);
          assert_eq!(start.as_ptr(), start2.as_ptr());
          assert_eq!(line, 1);
        }
        _ => panic!("Wrong error: {error:?}"),
      }

      assert_none!(scanner.next());
    }}
}
