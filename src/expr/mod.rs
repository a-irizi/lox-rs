mod literal;
mod operators;

use literal::LiteralToken;
use operators::{BinaryOperator, UnaryOperator};

pub enum Expr<'src> {
  Binary { left: Box<Expr<'src>>, operator: BinaryOperator<'src>, right: Box<Expr<'src>> },
  Grouping(Box<Expr<'src>>),
  Unary { operator: UnaryOperator<'src>, right: Box<Expr<'src>> },
  Literal(LiteralToken<'src>),
}

impl<'src> Expr<'src> {
  pub fn binary(left: Expr<'src>, operator: BinaryOperator<'src>, right: Expr<'src>) -> Self {
    Expr::Binary { left: Box::new(left), operator, right: Box::new(right) }
  }
  pub fn literal(literal: LiteralToken<'src>) -> Self {
    Expr::Literal(literal)
  }
  pub fn unary(operator: UnaryOperator<'src>, right: Expr<'src>) -> Self {
    Expr::Unary { operator, right: Box::new(right) }
  }
  pub fn grouping(expression: Expr<'src>) -> Self {
    Expr::Grouping(Box::new(expression))
  }
}

impl<'src> AsRef<Expr<'src>> for Expr<'src> {
  fn as_ref(&self) -> &Expr<'src> {
    self
  }
}

pub trait ExprFormatter {
  fn format(&self, expr: &Expr) -> String;
}

pub struct AstFormatter;
impl ExprFormatter for AstFormatter {
  fn format(&self, expr: &Expr) -> String {
    match expr {
      Expr::Binary { left, operator, right } => {
        format!("({} {} {})", operator, self.format(left), self.format(right))
      }
      Expr::Grouping(expression) => format!("(group {})", self.format(expression)),
      Expr::Unary { operator, right } => {
        format!("({} {})", operator, self.format(right))
      }
      Expr::Literal(literal_token) => format!("{literal_token}"),
    }
  }
}

pub struct RpnFormatter;
impl ExprFormatter for RpnFormatter {
  fn format(&self, expr: &Expr) -> String {
    match expr {
      Expr::Binary { left, operator, right } => {
        format!("{} {} {}", self.format(left), self.format(right), operator)
      }
      Expr::Grouping(expr) => self.format(expr),
      Expr::Unary { operator, right } => format!("{}{}", operator, self.format(right)),
      Expr::Literal(literal_token) => format!("{literal_token}"),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::token::{Token, TokenKind};

  use super::*;

  #[test]
  fn print_ast_binary() {
    let expr = Expr::binary(
      Expr::literal(Token::new(TokenKind::Number, Some("1"), 1).try_into().unwrap()),
      Token::new(TokenKind::Plus, None, 1).try_into().unwrap(),
      Expr::literal(Token::new(TokenKind::Number, Some("2"), 1).try_into().unwrap()),
    );
    let tests: Vec<(&dyn ExprFormatter, _, _)> = vec![
      (&AstFormatter, "(+ 1 2)", "wrong Lisp-like notation result"),
      (&RpnFormatter, "1 2 +", "wrong RPN result"),
    ];

    for (stringifer, expected, error) in tests {
      let ast = stringifer.format(&expr);
      println!("{ast}");
      assert_eq!(expected, ast, "{error}");
    }
  }

  #[test]
  fn print_ast_unary() {
    let expr = Expr::unary(
      Token::new(TokenKind::Minus, None, 1).try_into().unwrap(),
      Expr::literal(Token::new(TokenKind::Number, Some("2"), 1).try_into().unwrap()),
    );

    let tests: Vec<(&dyn ExprFormatter, _, _)> = vec![
      (&AstFormatter, "(- 2)", "wrong Lisp-like notation result"),
      (&RpnFormatter, "-2", "wrong RPN result"),
    ];

    for (stringifer, expected, error) in tests {
      let ast = stringifer.format(&expr);
      println!("{ast}");
      assert_eq!(expected, ast, "{error}");
    }
  }

  #[test]
  fn print_ast_grouping() {
    let expr = Expr::grouping(Expr::literal(
      Token::new(TokenKind::Number, Some("45.67"), 1).try_into().unwrap(),
    ));

    let tests: Vec<(&dyn ExprFormatter, _, _)> = vec![
      (&AstFormatter, "(group 45.67)", "wrong Lisp-like notation result"),
      (&RpnFormatter, "45.67", "wrong RPN result"),
    ];

    for (stringifer, expected, error) in tests {
      let ast = stringifer.format(&expr);
      println!("{ast}");
      assert_eq!(expected, ast, "{error}");
    }
  }

  #[test]
  fn print_ast_literal() {
    let expr = Expr::literal(Token::new(TokenKind::Number, Some("45.67"), 1).try_into().unwrap());

    let tests: Vec<(&dyn ExprFormatter, _, _)> = vec![
      (&AstFormatter, "45.67", "wrong Lisp-like notation result"),
      (&RpnFormatter, "45.67", "wrong RPN result"),
    ];

    for (stringifer, expected, error) in tests {
      let ast = stringifer.format(&expr);
      println!("{ast}");
      assert_eq!(expected, ast, "{error}");
    }
  }

  #[test]
  fn print_ast_mixed() {
    let expr = Expr::binary(
      Expr::unary(
        Token::new(TokenKind::Minus, None, 1).try_into().unwrap(),
        Expr::literal(Token::new(TokenKind::Number, Some("123"), 1).try_into().unwrap()),
      ),
      Token::new(TokenKind::Star, None, 1).try_into().unwrap(),
      Expr::grouping(Expr::literal(
        Token::new(TokenKind::Number, Some("45.67"), 1).try_into().unwrap(),
      )),
    );

    let tests: Vec<(&dyn ExprFormatter, _, _)> = vec![
      (&AstFormatter, "(* (- 123) (group 45.67))", "wrong Lisp-like notation result"),
      (&RpnFormatter, "-123 45.67 *", "wrong RPN result"),
    ];

    for (stringifer, expected, error) in tests {
      let ast = stringifer.format(&expr);
      println!("{ast}");
      assert_eq!(expected, ast, "{error}");
    }
  }
}
