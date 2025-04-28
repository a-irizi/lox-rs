use super::{Expr, ExprVisitor};

/// Abstract Syntax Tree (AST) formatter in List style format.
pub struct AstFormatter;
impl ExprVisitor for AstFormatter {
  fn visit(&self, expr: &Expr) -> String {
    match expr {
      Expr::Binary { left, operator, right } => {
        format!("({} {} {})", operator, self.visit(left), self.visit(right))
      }
      Expr::Grouping(expression) => format!("(group {})", self.visit(expression)),
      Expr::Unary { operator, right } => {
        format!("({} {})", operator, self.visit(right))
      }
      Expr::Literal(literal_token) => format!("{literal_token}"),
    }
  }
}

/// Abstract Syntax Tree (AST) formatter in Reverse Polish Notation (RPN) format.
pub struct RpnFormatter;
impl ExprVisitor for RpnFormatter {
  fn visit(&self, expr: &Expr) -> String {
    match expr {
      Expr::Binary { left, operator, right } => {
        format!("{} {} {}", self.visit(left), self.visit(right), operator)
      }
      Expr::Grouping(expr) => self.visit(expr),
      Expr::Unary { operator, right } => format!("{}{}", operator, self.visit(right)),
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
    let tests: Vec<(&dyn ExprVisitor, _, _)> = vec![
      (&AstFormatter, "(+ 1 2)", "wrong Lisp-like notation result"),
      (&RpnFormatter, "1 2 +", "wrong RPN result"),
    ];

    for (stringifer, expected, error) in tests {
      let ast = stringifer.visit(&expr);
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

    let tests: Vec<(&dyn ExprVisitor, _, _)> = vec![
      (&AstFormatter, "(- 2)", "wrong Lisp-like notation result"),
      (&RpnFormatter, "-2", "wrong RPN result"),
    ];

    for (stringifer, expected, error) in tests {
      let ast = stringifer.visit(&expr);
      println!("{ast}");
      assert_eq!(expected, ast, "{error}");
    }
  }

  #[test]
  fn print_ast_grouping() {
    let expr = Expr::grouping(Expr::literal(
      Token::new(TokenKind::Number, Some("45.67"), 1).try_into().unwrap(),
    ));

    let tests: Vec<(&dyn ExprVisitor, _, _)> = vec![
      (&AstFormatter, "(group 45.67)", "wrong Lisp-like notation result"),
      (&RpnFormatter, "45.67", "wrong RPN result"),
    ];

    for (stringifer, expected, error) in tests {
      let ast = stringifer.visit(&expr);
      println!("{ast}");
      assert_eq!(expected, ast, "{error}");
    }
  }

  #[test]
  fn print_ast_literal() {
    let expr = Expr::literal(Token::new(TokenKind::Number, Some("45.67"), 1).try_into().unwrap());

    let tests: Vec<(&dyn ExprVisitor, _, _)> = vec![
      (&AstFormatter, "45.67", "wrong Lisp-like notation result"),
      (&RpnFormatter, "45.67", "wrong RPN result"),
    ];

    for (stringifer, expected, error) in tests {
      let ast = stringifer.visit(&expr);
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

    let tests: Vec<(&dyn ExprVisitor, _, _)> = vec![
      (&AstFormatter, "(* (- 123) (group 45.67))", "wrong Lisp-like notation result"),
      (&RpnFormatter, "-123 45.67 *", "wrong RPN result"),
    ];

    for (stringifer, expected, error) in tests {
      let ast = stringifer.visit(&expr);
      println!("{ast}");
      assert_eq!(expected, ast, "{error}");
    }
  }
}
