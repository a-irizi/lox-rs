use thiserror::Error;

use crate::scanner;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  // Modules
  #[error("failed to parse source")]
  Scanner(
    #[from]
    #[source]
    scanner::Error,
  ),
  // External
  #[error("failed to read source file")]
  SourceFileReading(
    #[from]
    #[source]
    std::io::Error,
  ),
}
