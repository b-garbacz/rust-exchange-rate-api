use std::env;
use tokio;
mod arguments;
mod request_handler;
use request_handler::ApiRequest;
use request_handler::FetchResult;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match arguments::parse() {
        Ok((source, target, decimal_value)) => {

            let api_key = match env::var_os("API_KEY") 
            {
                Some(_str) => _str.into_string().unwrap(),
                None => panic!("$API_KEY is not set")
            };

            if !source.is_empty() && !target.is_empty() && !decimal_value.is_zero()
            {
                let request = request_handler::StandardRequest::new(&source, &target, decimal_value, api_key);
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
                let request = request_handler::SupportedCodes::new(api_key);
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
                let request = request_handler::ConversionRates::new(source, api_key);
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
