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
pub enum Expr<'src> {
  Binary { left: Box<Expr<'src>>, operator: BinaryOperator<'src>, right: Box<Expr<'src>> },
  Ternary { condition: Box<Expr<'src>>, then_branch: Box<Expr<'src>>, else_branch: Box<Expr<'src>> },
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

  /// creates a new ternary expression.
  ///
  /// # Returns
  /// A new ternary expression.
  pub fn ternary(condition: Expr<'src>, then_branch: Expr<'src>, else_branch: Expr<'src>) -> Self {
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
