mod arguments;
mod request_handler;
mod models;
use crate::models::FetchResult;
use crate::request_handler::ApiRequest;

use std::env;
use tokio;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match arguments::parse() {
        Ok((source, target, decimal_value)) => {

            let api_key = match env::var_os("API_KEY")
            {
                Some(api_key_string) => api_key_string.into_string().unwrap(),
                None => panic!("$API_KEY is not set")
            };

            if !source.is_empty() && !target.is_empty() && !decimal_value.is_zero()
            {
                let prefix = "https://v6.exchangerate-api.com/".to_owned();
                let request = request_handler::StandardRequest::new(&source, &target, &decimal_value, &api_key, &prefix);
                match request.fetch().await {
                    Ok(Some(FetchResult::Decimal(convertion_result))) => {
                        println!("Conversion from {} -> {} = {:?}", source, target, convertion_result);
                    },
                    Ok(None) => {
                        println!("Incorrect Source/Target");
                    },
                    Err(e) => {
                        eprintln!("Error fetching conversion rate: {}", e);
                    },
                    _ => (),
                }
            } 
            else if source.is_empty() && target.is_empty() && decimal_value.is_zero()
            {
                let prefix = "https://v6.exchangerate-api.com/".to_string();
                let request = request_handler::SupportedCodes::new(&api_key, &prefix);
                match request.fetch().await {
                    Ok(Some(FetchResult::VecString(supported_codes))) => {
                        for currency_pair in supported_codes {
                            println!("Currency Code: {:?}, Currency Name: {:?}", currency_pair[0], currency_pair[1]);
                        }
                    },
                    Ok(None) => println!("No data available."),
                    Err(e) => eprintln!("Error: {}", e),
                    _ => (),
                }
            }
            else if !source.is_empty()  && target.is_empty() && decimal_value.is_zero() {
                let prefix = "https://v6.exchangerate-api.com/".to_string();
                let request = request_handler::ConversionRates::new(&source, &api_key, &prefix);
                match request.fetch().await {
                    Ok(Some(FetchResult::HashMapRates(conversion_rates))) => {
                        for element in conversion_rates
                        {
                            println!("Currency {},  convertion rate {}",element.0, element.1);
                        }
                    }
                    Ok(None) => println!("No data available."),
                    Err(e) => eprintln!("Error: {}", e),
                    _ => (),
                }
            }
        },
        Err(e) => {
            eprint!("{}", e);
        }
    }
    Ok(())
}
