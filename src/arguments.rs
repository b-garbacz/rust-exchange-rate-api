use rust_decimal::{prelude::Zero, Decimal};
use core::fmt;
use std::{env, str::FromStr};

#[derive(Debug)]
pub enum ArgsError 
{
    HelpRequested,
    InvalidNumberOfArguments,
    ParseError,
}

impl fmt::Display for ArgsError 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match *self 
        {
            ArgsError::HelpRequested => write!(f, "
            
            -----------------------------------------------------

            Default mode:
            ./exchange-rate-api <source> <target> <decimal_value>
            Provide three arguments: 
            <source> - the currency you are converting from,
            <target> - the currency you are converting to,
            <decimal_value> - the amount to be converted.

            Example: ./exchange-rate-api USD PLN 100.50

            -----------------------------------------------------

            List all available currencies:
            ./exchange-rate-api --codes

            -----------------------------------------------------
            
            List all available currencies and the current exchange rates against a single currency:
            ./exchange-rate-api <source>

            Provide one argument:
            <source> - the currency you are converting from,
            Example: ./exchange-rate-api USD

            -----------------------------------------------------
            "),
    
            ArgsError::InvalidNumberOfArguments => write!(f, "Invalid number of arguments! Use --help to display help message."),
            ArgsError::ParseError => write!(f, "Error with parsing 3th argument. Please enter floating number\n"),
        }
    }
}

pub fn parse() -> Result<(String, String, Decimal), ArgsError>
{
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--help"
    {
        return Err(ArgsError::HelpRequested);
    }
    else if args.len() == 4
    {

        let source_currency_code = &args[1];
        let target_currency_code = &args[2];
        let decimal_value = match Decimal::from_str(&args[3])
        {
            Ok(val) => val,
            Err(_) =>  return Err(ArgsError::ParseError),
        };

        return Ok((source_currency_code.to_string(), target_currency_code.to_string(), decimal_value))

    }
    else if args.len() == 2 && args[1] == "--codes" 
    {
        return Ok((String::new(), String::new(), Decimal::zero()));
    }
    else if args.len() == 2 
    {
        let source_currency_code = &args[1];
        return Ok((source_currency_code.to_string(), String::new(), Decimal::zero()))
    }

    return Err(ArgsError::InvalidNumberOfArguments);
}

 