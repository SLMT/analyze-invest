
use std::io::Write;

use colored::*;

use crate::error::{AnalyzerError, AnalyzerResult};

pub fn create(fund_id: &str) -> AnalyzerResult<()> {
  // TODO: check duplicate fund
  
  display_welcome_message(fund_id);
  check_id_format(fund_id)?;
  let fund_name = request_fund_name()?;

  // TODO: get intial portfolio
  // TODO: show the result and asking if it is needed to change
  // TODO: show ending message (where the data are saved)

  todo!()
}

fn display_welcome_message(fund_id: &str) {
  println!("Creating a new fund: {}", fund_id.yellow());
}

fn check_id_format(fund_id: &str) -> AnalyzerResult<()> {
  for ch in fund_id.chars() {
    if !ch.is_ascii_alphanumeric() && ch != '_' {
      return Err(AnalyzerError::FundIdFormatError);
    }
  }

  Ok(())
}

fn request_fund_name() -> AnalyzerResult<String> {
  // Promot
  print!("Please enter the name of the fund: ");
  std::io::stdout().flush()?;

  // Inputs
  let mut input = String::new();
  std::io::stdin().read_line(&mut input)?;
  let input = input.trim();

  // Show the name
  println!("The name is `{}`", input.yellow());

  Ok(input.to_owned())
}

fn request_initial_portfolio() -> AnalyzerResult<()> {
  todo!();
}

fn request_cash_amount() -> AnalyzerResult<()> {
  todo!();
}
