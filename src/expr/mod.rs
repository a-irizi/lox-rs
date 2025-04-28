mod binary_operator;
mod error;
mod formatter;
mod literal;
mod unary_operator;

use self::{
  binary_operator::BinaryOperator,
  error::{Error, Result},
  literal::Literal,
  unary_operator::UnaryOperator,
};

/// represents an expression in the source code.
pub enum Expr<'src> {
  Binary { left: Box<Expr<'src>>, operator: BinaryOperator<'src>, right: Box<Expr<'src>> },
  Grouping(Box<Expr<'src>>),
  Unary { operator: UnaryOperator<'src>, right: Box<Expr<'src>> },
  Literal(Literal<'src>),
}

impl<'src> Expr<'src> {
  /// creates a new binary expression.
  ///
  /// # Returns
  /// A new binary expression.
  pub fn binary(left: Expr<'src>, operator: BinaryOperator<'src>, right: Expr<'src>) -> Self {
    Expr::Binary { left: Box::new(left), operator, right: Box::new(right) }
  }
  /// creates a new literal expression.
  ///
  /// # Returns
  /// A new literal expression.
  pub fn literal(literal: Literal<'src>) -> Self {
    Expr::Literal(literal)
  }
  /// creates a new unary expression.
  ///
  /// # Returns
  /// A new unary expression.
  pub fn unary(operator: UnaryOperator<'src>, right: Expr<'src>) -> Self {
    Expr::Unary { operator, right: Box::new(right) }
  }
  /// creates a new grouping expression.
  ///
  /// # Returns
  /// A new grouping expression.
  pub fn grouping(expression: Expr<'src>) -> Self {
    Expr::Grouping(Box::new(expression))
  }
}

/// `ExprVisitor` is a trait that defines a visitor pattern for expressions.
pub trait ExprVisitor {
  fn visit(&self, expr: &Expr) -> String;
}
