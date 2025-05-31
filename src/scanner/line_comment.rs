use super::Scanner;

#[allow(clippy::extra_unused_lifetimes)]
impl Scanner<'_> {
  /// swallows a line comment.
  pub(super) fn line_comment(&mut self) {
    if let Some(new_line_index) = self.rest.find('\n') {
      self.line += 1;
      self.rest = &self.rest[new_line_index + 1..];
    } else {
      self.rest = "";
    }
  }
}

#[cfg(test)]
mod tests {
  use claims::assert_none;
  use rstest::rstest;

  use super::*;

  #[rstest(source)]
  #[case::single("// This is a comment")]
  #[case::multiple("// This is a comment")]
  fn lex_line_comment_ok(source: &str) {
    let mut scanner = Scanner::new(source);
    assert_none!(scanner.next());
  }
}
