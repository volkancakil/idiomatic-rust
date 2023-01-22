use std::num::ParseFloatError;

use self::thiserror::Error;

extern crate thiserror;

#[derive(Error, Debug)]
pub enum MoneyError {
    #[error("Invalid input: {0}")]
    ParseAmount(ParseFloatError),
    #[error("{0}")]
    ParseFormatting(String),
    #[error("{0}")]
    ParseCurrency(String)
}

impl From<ParseFloatError> for MoneyError {
    fn from(e: ParseFloatError) -> Self {
        MoneyError::ParseAmount(e)
    }
}