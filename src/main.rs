/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

extern crate currency;
use currency::{error::MoneyError, money::Money};

fn main() {
    let money = parse_money("100 $");
    println!("Hello, {:#?}!", money);

    let money = "140 tl".parse::<Money>();
    println!("Hello, {:#?}!", money);

}

fn parse_money(input: &str) -> Result<Money, MoneyError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    match parts[..] {
        [amount, currency] => Ok(Money::new(amount.parse()?, currency.parse()?)),
        _ => Err(MoneyError::ParseFormatting("Expected amount and currency".into()))
    }
}