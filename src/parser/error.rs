use thiserror::Error;

use crate::expr::Expr;
use crate::token::Token;

pub type Result<'src> = core::result::Result<Expr, Error<'src>>;

#[derive(Debug, Error)]
pub enum Error<'src> {
  #[error("Expected ')' after expression")]
  GroupingNotClosed { right_paren: Token<'src>, expr: Expr },
  #[error("Expected expression after '('")]
  GroupingMissingExpression { right_paren: Token<'src> },
  #[error("Expected expression '?'")]
  TernaryMissingThenBranch { question: Expr, then_operator: Token<'src> },
  #[error("Expected ':' after expression")]
  TernaryMissingElseOperator { question: Expr, then_operator: Token<'src>, then_branch: Expr },
  #[error("Expected expression after ':'")]
  TernaryMissingElseBranch {
    question: Expr,
    then_operator: Token<'src>,
    then_branch: Expr,
    else_operator: Token<'src>,
  },
}
