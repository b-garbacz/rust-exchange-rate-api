use exchange_rate_api::request_handler::{ApiRequest, ConversionRates, StandardRequest, SupportedCodes};
use rust_decimal::Decimal;
use tokio;
use std::env;

#[tokio::test]
async fn test_fetch_with_invalid_api_key() {
    let invalid_api_key = "invalid_key".to_string();
    let usd = "USD".to_string();
    let eur= "EUR".to_string();
    let dec = Decimal::new(1,0);

    let prefix = "https://v6.exchangerate-api.com/".to_string();
    let standard_request = StandardRequest::new(&usd, &eur, &dec, &invalid_api_key, &prefix);
    let standard_request_result = standard_request.fetch().await;
    assert!(standard_request_result.is_err(), "Expected an error for the invalid API key");

    let conversion_rates_request =  ConversionRates::new(&usd, &invalid_api_key, &prefix);
    let conversion_rates_result = conversion_rates_request.fetch().await;
    assert!(conversion_rates_result.is_err(), "Expected an error for the invalid API key");

    
    let conversion_codes_request =  SupportedCodes::new(&invalid_api_key, &prefix);
    let conversions_codes_result = conversion_codes_request.fetch().await;
    assert!(conversions_codes_result.is_err(), "Expected an error for the invalid API key");
}

#[tokio::test]
async fn test_fetch_with_malformed_request(){

    let api_key = match env::var_os("API_KEY")
            {
                Some(_str) => _str.into_string().unwrap(),
                None => panic!("$API_KEY is not set")
            };
    let usd = "USDD".to_string();
    let eur= "EUR".to_string();
    let dec = Decimal::new(1,0);

    let prefix = "https://v6.exchangerate-api.com/".to_string();
    let standard_request = StandardRequest::new(&usd, &eur, &dec, &api_key, &prefix);
    let standard_request_result = standard_request.fetch().await;
    assert!(standard_request_result.is_err(), "Malformed request.");

    let conversion_rates_request =  ConversionRates::new(&usd, &api_key, &prefix);
    let conversion_rates_result = conversion_rates_request.fetch().await;
    assert!(conversion_rates_result.is_err(), "Malformed request.");

}
