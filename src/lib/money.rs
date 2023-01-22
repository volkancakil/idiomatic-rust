use std::str::FromStr;

use crate::error::MoneyError;

 #[derive(Debug)]
pub enum Currency {
    Dollar,
    Euro,
    TL
 }

 impl FromStr for Currency {
     type Err = MoneyError;
     fn from_str(s: &str) -> Result<Self, Self::Err> {
         match s.to_lowercase().as_ref() {
             "dollar" | "$" => Ok(Currency::Dollar),
             "euro" | "€" => Ok(Currency::Euro),
             "tl" | "₺" => Ok(Currency::TL),
             _ => Err(MoneyError::ParseCurrency("Unknown currency".into()))
         }
     }
 }

#[derive(Debug)]
pub struct Money {
    amount: f32,
    currency: Currency
}

impl Money {
    pub fn new(amount: f32, currency: Currency) -> Self {
        Money { amount, currency }
    }
}

impl FromStr for Money {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect(); 
        match parts[..] {
            [amount, currency] => Ok(Money::new(amount.parse()?, currency.parse()?)),
            _ => Err(MoneyError::ParseFormatting("Expecting amount and currency".into()))
        }
    }
}