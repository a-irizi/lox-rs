use super::{Error, Result, Scanner};

#[allow(clippy::extra_unused_lifetimes)]
impl<'src> Scanner<'src> {
  /// swallows a block comment.
  ///
  /// # Errors
  ///
  /// - [`MissingBlockCommentTerminator`](crate::scanner::error::Error::MissingBlockCommentTerminator): The block comment was not terminated.
  pub(super) fn block_comment(&mut self) -> Result<'src, ()> {
    let line = self.line;
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

    match comment_end_index {
      Some(comment_end_index) => {
        self.rest = &self.rest[comment_end_index + 1..];
        Ok(())
      }
      None => Err(Error::MissingBlockCommentTerminator { start: &self.rest[..2], line }),
    }
  }
}

#[cfg(test)]
mod tests {
  use claims::{assert_err, assert_matches, assert_none, assert_some};
  use proptest::{
    prelude::{Strategy, any},
    prop_compose, proptest,
  };
  use rstest::rstest;

  use super::*;

  #[rstest(source)]
  #[case::single_line_block_comment("/* This is a comment */")]
  #[case::multi_line_block_comment(
    "/* This is a comment
        as you can see
        this is a block comment
        */"
  )]
  fn lex_block_comment_ok(source: &str) {
    let mut scanner = Scanner::new(source);
    assert_none!(scanner.next());
  }

  #[rstest(source)]
  #[case::single_line_block_comment("/* This is a comment *")]
  #[case::multi_line_block_comment(
    "/* This is a comment
        as you can see
        this is a block comment
        "
  )]
  fn lex_unterminated_block_comment_err(source: &str) {
    let mut scanner = Scanner::new(source);
    let next = assert_some!(scanner.next());
    let err = assert_err!(next);

    assert_matches!(err, Error::MissingBlockCommentTerminator { start: "/*", line: 1 });
    match err {
      Error::MissingBlockCommentTerminator { start, line } => {
        let comment_start = &source[..2];
        assert_eq!(comment_start.len(), start.len());
        assert_eq!(comment_start.as_ptr(), start.as_ptr())
      }
      _ => unreachable!(),
    }
  }

  fn arb_block_comment_content() -> impl Strategy<Value = String> {
    any::<String>().prop_map(|content| content.replace("*/", "*)/"))
  }

  prop_compose! {
    fn arb_block_comment()(content in arb_block_comment_content()) -> String {
      format!("/*{content}*/")
    }
  }

  proptest! {
    #[test]
    fn arb_comment_ok(source in arb_block_comment()) {
      let mut scanner = Scanner::new(&source);
      assert_none!(scanner.next());
    }
  }
}
