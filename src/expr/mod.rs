mod error;
mod formatter;
mod terminal;

pub use self::{
  error::{Error, Result},
  formatter::{AstFormatter, RpnFormatter},
  terminal::{
    BinaryOperator, CommaOperator, ComparisonOperator, FactorOperator, Literal, TermOperator,
    Terminal, TernaryElseOperator, TernaryThenOperator, UnaryOperator,
  },
};

/// represents an expression in the source code.
#[derive(Debug)]
pub enum Expr {
  Binary { left: Box<Expr>, operator: BinaryOperator, right: Box<Expr> },
  Ternary { condition: Box<Expr>, then_branch: Box<Expr>, else_branch: Box<Expr> },
  Grouping(Box<Expr>),
  Unary { operator: UnaryOperator, right: Box<Expr> },
  Literal(Literal),
}

impl Expr {
  /// creates a new binary expression.
  ///
  /// # Returns
  /// A new binary expression.
  pub fn binary(left: Expr, operator: BinaryOperator, right: Expr) -> Self {
    Expr::Binary { left: Box::new(left), operator, right: Box::new(right) }
  }

  /// creates a new ternary expression.
  ///
  /// # Returns
  /// A new ternary expression.
  pub fn ternary(condition: Expr, then_branch: Expr, else_branch: Expr) -> Self {
    Expr::Ternary {
      condition: Box::new(condition),
      then_branch: Box::new(then_branch),
      else_branch: Box::new(else_branch),
    }
  }
  /// creates a new literal expression.
  ///
  /// # Returns
  /// A new literal expression.
  pub fn literal(literal: Literal) -> Self {
    Expr::Literal(literal)
  }
  /// creates a new unary expression.
  ///
  /// # Returns
  /// A new unary expression.
  pub fn unary(operator: UnaryOperator, right: Expr) -> Self {
    Expr::Unary { operator, right: Box::new(right) }
  }
  /// creates a new grouping expression.
  ///
  /// # Returns
  /// A new grouping expression.
  pub fn grouping(expression: Expr) -> Self {
    Expr::Grouping(Box::new(expression))
  }
}

/// `ExprVisitor` is a trait that defines a visitor pattern for expressions.
pub trait ExprVisitor<T> {
  fn visit(&self, expr: &Expr) -> T;
}
