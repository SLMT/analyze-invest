
use thiserror::Error;

pub type AnalyzerResult<T> = Result<T, AnalyzerError>;

#[derive(Error, Debug)]
pub enum AnalyzerError {

}
