use super::Scanner;

#[allow(clippy::extra_unused_lifetimes)]
impl<'a> Scanner<'_> {
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

  use super::*;

  #[test]
  fn lex_one_line_comment_ok() {
    let mut scanner = Scanner::new("// This is a comment");
    assert_none!(scanner.next());
  }

  #[test]
  fn lex_multiple_line_comments_ok() {
    let source = "// This is a comment
  // This is another comment
  // This is a third comment";
    let mut scanner = Scanner::new(source);
    assert_none!(scanner.next());
  }
}
