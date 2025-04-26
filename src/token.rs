use std::fmt::Display;

#[derive(Debug)]
pub struct Token<'src> {
  pub kind: TokenKind,
  pub lexeme: Option<&'src str>,
  pub line: usize,
}

impl<'src> Token<'src> {
  pub fn new(kind: TokenKind, lexeme: Option<&'src str>, line: usize) -> Self {
    Self { kind, lexeme, line }
  }
}

impl Display for Token<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.kind {
      TokenKind::Comma => write!(f, ","),
      TokenKind::Dot => write!(f, "."),
      TokenKind::LeftBrace => write!(f, "{{"),
      TokenKind::LeftParen => write!(f, "("),
      TokenKind::Minus => write!(f, "-"),
      TokenKind::Plus => write!(f, "+"),
      TokenKind::RightBrace => write!(f, "}}"),
      TokenKind::RightParen => write!(f, ")"),
      TokenKind::SemiColon => write!(f, ";"),
      TokenKind::Slash => write!(f, "/"),
      TokenKind::Star => write!(f, "*"),
      TokenKind::Bang => write!(f, "!"),
      TokenKind::BangEqual => write!(f, "!="),
      TokenKind::Equal => write!(f, "="),
      TokenKind::EqualEqual => write!(f, "=="),
      TokenKind::Greater => write!(f, ">"),
      TokenKind::GreaterEqual => write!(f, ">="),
      TokenKind::Less => write!(f, "<"),
      TokenKind::LessEqual => write!(f, "<="),
      TokenKind::Identifier | TokenKind::Number => write!(f, "{}", self.lexeme.unwrap()),
      TokenKind::String => write!(f, "{:?}", self.lexeme.unwrap()),
      TokenKind::And => write!(f, "&&"),
      TokenKind::Class => write!(f, "class"),
      TokenKind::Else => write!(f, "else"),
      TokenKind::False => write!(f, "false"),
      TokenKind::Fun => write!(f, "fun"),
      TokenKind::For => write!(f, "for"),
      TokenKind::If => write!(f, "if"),
      TokenKind::Nil => write!(f, "nil"),
      TokenKind::Or => write!(f, "or"),
      TokenKind::Print => write!(f, "print"),
      TokenKind::Return => write!(f, "return"),
      TokenKind::Super => write!(f, "super"),
      TokenKind::This => write!(f, "this"),
      TokenKind::True => write!(f, "true"),
      TokenKind::Var => write!(f, "var"),
      TokenKind::While => write!(f, "while"),
    }
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
