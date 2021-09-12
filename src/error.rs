
use thiserror::Error;

pub type AnalyzerResult<T> = Result<T, AnalyzerError>;

#[derive(Error, Debug)]
pub enum AnalyzerError {
  #[error("the fund ID must be in 0-9, a-z, A-Z, or '_'.")]
  FundIdFormatError,
  #[error("io error")]
  Io { 
    #[from]
    source: std::io::Error
  }
}
