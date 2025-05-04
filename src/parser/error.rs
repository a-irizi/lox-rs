use thiserror::Error;

use crate::expr::Expr;
use crate::token::Token;

pub type Result<'src> = core::result::Result<Expr<'src>, Error<'src>>;

#[derive(Debug, Error)]
pub enum Error<'src> {
  #[error("Expected ')' after expression")]
  GroupingNotClosed { right_paren: Token<'src>, expr: Expr<'src> },
  #[error("Expected expression after '('")]
  GroupingMissingExpression { right_paren: Token<'src> },
  #[error("Expected expression '?'")]
  TernaryMissingThenBranch { question: Expr<'src>, then_operator: Token<'src> },
  #[error("Expected ':' after expression")]
  TernaryMissingElseOperator {
    question: Expr<'src>,
    then_operator: Token<'src>,
    then_branch: Expr<'src>,
  },
  #[error("Expected expression after ':'")]
  TernaryMissingElseBranch {
    question: Expr<'src>,
    then_operator: Token<'src>,
    then_branch: Expr<'src>,
    else_operator: Token<'src>,
  },
}
