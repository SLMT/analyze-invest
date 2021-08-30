
use colored::*;

use crate::error::AnalyzerResult;

pub fn create(fund_id: &str) -> AnalyzerResult<()> {
  // TODO: check duplicate fund
  
  display_welcome_message(&fund_id);

  // TODO: check fund ID format (1-9, a-z, '_')

  // TODO: get fund name
  // TODO: get intial portfolio
  // TODO: show the result and asking if it is needed to change
  // TODO: show ending message (where the data are saved)

  todo!()
}

fn display_welcome_message(fund_id: &str) {
  println!("Creating a new fund: {}", fund_id.yellow());
}
