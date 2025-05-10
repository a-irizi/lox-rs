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
      TokenKind::Identifier => write!(f, "{}", self.lexeme.unwrap()),
      ref kind => write!(f, "{kind}"),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
  // Single-character tokens
  Colon,
  Comma,
  Dot,
  LeftBrace,
  LeftParen,
  Minus,
  Plus,
  Question,
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
  String(String),
  Number(f64),

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

impl Display for TokenKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TokenKind::And => write!(f, "&&"),
      TokenKind::Bang => write!(f, "!"),
      TokenKind::BangEqual => write!(f, "!="),
      TokenKind::Class => write!(f, "class"),
      TokenKind::Colon => write!(f, ":"),
      TokenKind::Comma => write!(f, ","),
      TokenKind::Dot => write!(f, "."),
      TokenKind::Else => write!(f, "else"),
      TokenKind::Equal => write!(f, "="),
      TokenKind::EqualEqual => write!(f, "=="),
      TokenKind::False => write!(f, "false"),
      TokenKind::For => write!(f, "for"),
      TokenKind::Fun => write!(f, "fun"),
      TokenKind::Greater => write!(f, ">"),
      TokenKind::GreaterEqual => write!(f, ">="),
      TokenKind::Identifier => write!(f, "IDENT"),
      TokenKind::Number(n) => write!(f, "{}", n),
      TokenKind::If => write!(f, "if"),
      TokenKind::LeftBrace => write!(f, "{{"),
      TokenKind::LeftParen => write!(f, "("),
      TokenKind::Less => write!(f, "<"),
      TokenKind::LessEqual => write!(f, "<="),
      TokenKind::Minus => write!(f, "-"),
      TokenKind::Nil => write!(f, "nil"),
      TokenKind::Or => write!(f, "or"),
      TokenKind::Plus => write!(f, "+"),
      TokenKind::Print => write!(f, "print"),
      TokenKind::Question => write!(f, "?"),
      TokenKind::Return => write!(f, "return"),
      TokenKind::RightBrace => write!(f, "}}"),
      TokenKind::RightParen => write!(f, ")"),
      TokenKind::SemiColon => write!(f, ";"),
      TokenKind::Slash => write!(f, "/"),
      TokenKind::Star => write!(f, "*"),
      TokenKind::String(s) => write!(f, "{:?}", s),
      TokenKind::Super => write!(f, "super"),
      TokenKind::This => write!(f, "this"),
      TokenKind::True => write!(f, "true"),
      TokenKind::Var => write!(f, "var"),
      TokenKind::While => write!(f, "while"),
    }
  }
}
