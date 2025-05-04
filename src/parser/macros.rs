/// return Expr in <code>Option<Result<Expr>></code> or else return.
macro_rules! expr_or_return {
  ($e:expr) => {
    match $e? {
      Ok(expr) => expr,
      Err(e) => return Some(Err(e)),
    }
  };
}

pub(super) use expr_or_return;
