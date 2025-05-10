use crate::token::{Token, TokenKind};

use super::Scanner;

impl<'a> Scanner<'a> {
  /// lexes a number token.
  pub(super) fn number(&mut self) -> Token<'a> {
    // index of the dot in the number, if any.
    let dot_index = match self.rest.char_indices().find(|(_, c)| !c.is_ascii_digit()) {
      // rest of input is all made of digits
      None => {
        let token =
          Token::new(
            TokenKind::Number(self.rest.parse().unwrap_or_else(|_| {
              panic!("Failed to parse integer lexeme {:?} into f64", self.rest)
            })),
            Some(self.rest),
            self.line,
          );
        self.rest = "";
        return token;
      }
      Some((index, c)) => {
        if c != '.' || self.rest[index + 1..].is_empty() {
          let lexeme = &self.rest[..index];
          self.rest = &self.rest[index..];
          return Token::new(
            TokenKind::Number(lexeme.parse().unwrap_or_else(|_| {
              panic!("Failed to parse integer lexeme {:?} into f64", self.rest)
            })),
            Some(lexeme),
            self.line,
          );
        }
        index
      }
    };

    let before_dot = &self.rest[..dot_index];
    let after_dot = &self.rest[dot_index + 1..];

    let lexeme = match after_dot.char_indices().find(|(_, c)| !c.is_ascii_digit()) {
      None => self.rest,
      Some((0, _)) => before_dot,
      Some((fract_index, _)) => &self.rest[..=dot_index + fract_index],
    };

    self.rest = &self.rest[lexeme.len()..];

    Token::new(
      TokenKind::Number(lexeme.parse().unwrap_or_else(|_| {
        panic!("Failed to parse floating point number lexeme {lexeme:?} into f64")
      })),
      Some(lexeme),
      self.line,
    )
  }
}

#[cfg(test)]
mod tests {
  use claims::{assert_matches, assert_none};

  use super::*;

  #[test]
  fn test_number_parsing() {
    // Test cases organized by category
    let source = "
          123          // simple integer
          45.67        // simple decimal
          .89          // leading dot (should be separate tokens)
          12.          // trailing dot
          34.56x       // decimal with trailing char
          78.9.0       // multiple dots
          0.0          // zero cases
          123e45       // scientific notation (not supported)
          12_34        // underscores (not supported)
          12.34.56.78  // multiple dots
          56.          // trailing dot
          .            // single dot
          999999.999999 // large numbers
          12.3.4       // invalid decimal
      ";

    let mut scanner = Scanner::new(source);

    // Expected results in order (kind, lexeme, line)
    let expected = [
      (TokenKind::Number(123f64), Some("123"), 2),
      (TokenKind::Number(45.67), Some("45.67"), 3),
      (TokenKind::Dot, None, 4),
      (TokenKind::Number(89f64), Some("89"), 4),
      (TokenKind::Number(12f64), Some("12"), 5),
      (TokenKind::Dot, None, 5),
      (TokenKind::Number(34.56), Some("34.56"), 6),
      (TokenKind::Identifier, Some("x"), 6),
      (TokenKind::Number(78.9), Some("78.9"), 7),
      (TokenKind::Dot, None, 7),
      (TokenKind::Number(0.0), Some("0"), 7),
      (TokenKind::Number(0.0), Some("0.0"), 8),
      (TokenKind::Number(123f64), Some("123"), 9),
      (TokenKind::Identifier, Some("e45"), 9),
      (TokenKind::Number(12f64), Some("12"), 10),
      (TokenKind::Identifier, Some("_34"), 10),
      (TokenKind::Number(12.34f64), Some("12.34"), 11),
      (TokenKind::Dot, None, 11),
      (TokenKind::Number(56.78), Some("56.78"), 11),
      (TokenKind::Number(56f64), Some("56"), 12),
      (TokenKind::Dot, None, 12),
      (TokenKind::Dot, None, 13),
      (TokenKind::Number(999_999.999_999), Some("999999.999999"), 14),
      (TokenKind::Number(12.3), Some("12.3"), 15),
      (TokenKind::Dot, None, 15),
      (TokenKind::Number(4f64), Some("4"), 15),
    ];

    // Verify all expected tokens
    for (i, (expected_kind, expected_lexeme, expected_line)) in expected.into_iter().enumerate() {
      assert_matches!(
          scanner.next(),
          Some(Ok(Token { kind, lexeme, line })) if
              kind == expected_kind &&
              lexeme == expected_lexeme &&
              line == expected_line,
          "Failed at token {}: expected {:?}", i+1, (expected_kind, expected_lexeme, expected_line)
      );
    }

    // Verify no extra tokens
    assert_none!(scanner.next(), "Unexpected extra tokens");

    // Additional edge case tests
    let edge_cases = [
      ("", vec![]), // empty input
      (".", vec![(TokenKind::Dot, None, 1)]),
      ("123", vec![(TokenKind::Number(123f64), Some("123"), 1)]),
      (
        "12.3.4",
        vec![
          (TokenKind::Number(12.3f64), Some("12.3"), 1),
          (TokenKind::Dot, None, 1),
          (TokenKind::Number(4f64), Some("4"), 1),
        ],
      ),
    ];

    for (input, expected) in edge_cases {
      let mut scanner = Scanner::new(input);
      for (i, (expected_kind, expected_lexeme, expected_line)) in expected.into_iter().enumerate() {
        assert_matches!(
            scanner.next(),
            Some(Ok(Token { kind, lexeme, line })) if
                kind == expected_kind &&
                lexeme == expected_lexeme &&
                line == expected_line,
            "Failed at token {}: expected {:?}", i+1, (expected_kind, expected_lexeme, expected_line)
        );
      }
      assert_none!(scanner.next(), "Extra tokens in edge case: {:?}", input);
    }
  }
}
