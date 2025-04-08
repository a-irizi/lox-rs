use super::{Error, Result, Scanner};

#[allow(clippy::extra_unused_lifetimes)]
impl Scanner<'_> {
  /// swallows a block comment.
  ///
  /// # Errors
  ///
  /// - [`MissingBlockCommentTerminator`](crates::scanner::error::Error::MissingBlockCommentTerminator): The block comment was not terminated.
  pub(super) fn block_comment(&mut self) -> Result<()> {
    let comment_end_index = self
      .rest
      .char_indices()
      // skip starting "/*"
      .skip(2)
      // count new lines
      .inspect(|(_, c)| {
        if *c == '\n' {
          self.line += 1;
        }
      })
      .find_map(|(index, c)| {
        let is_comment_terminator = c == '*' && self.rest[index + 1..].starts_with('/');
        if is_comment_terminator { Some(index + 1) } else { None }
      });

    if let Some(comment_end_index) = comment_end_index {
      self.rest = &self.rest[comment_end_index + 1..];
      Ok(())
    } else {
      Err(Error::MissingBlockCommentTerminator)
    }
  }
}

#[cfg(test)]
mod tests {
  use claims::{assert_matches, assert_none};

  use super::*;

  #[test]
  fn lex_single_line_block_comment_ok() {
    let mut scanner = Scanner::new("/* This is a comment */");
    assert_none!(scanner.next());
  }

  #[test]
  fn lex_single_line_block_comment_err() {
    let mut scanner = Scanner::new("/* This is a comment *");
    assert_matches!(scanner.next(), Some(Err(Error::MissingBlockCommentTerminator)));
  }

  #[test]
  fn lex_multi_line_block_comment_ok() {
    let mut scanner = Scanner::new(
      "/* This is a comment
      as you can see
      this is a block comment
      */",
    );
    assert_none!(scanner.next());
  }

  #[test]
  fn lex_multi_line_block_comment_err() {
    let mut scanner = Scanner::new(
      "/* This is a comment
      as you can see
      this is a block comment
      ",
    );
    assert_matches!(scanner.next(), Some(Err(Error::MissingBlockCommentTerminator)));
  }
}
