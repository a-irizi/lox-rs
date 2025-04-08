#[derive(Debug)]
pub struct Token<'a> {
  pub kind: TokenKind,
  pub lexeme: Option<&'a str>,
  pub line: usize,
}

impl<'a> Token<'a> {
  pub fn new(kind: TokenKind, lexeme: Option<&'a str>, line: usize) -> Self {
    Self { kind, lexeme, line }
  }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum TokenKind {
  // Single-character tokens
  Comma,
  Dot,
  LeftBrace,
  LeftParen,
  Minus,
  Plus,
  RightBrace,
  RightParen,
  SemiColon,
  Slash,
  Star,

  // One or two character tokens
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // Literals
  Identifier,
  String,
  Number,

  // Keywords
  And,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,
}
