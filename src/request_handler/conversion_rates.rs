use crate::models::{ApiResponse, FetchResult};
use crate::request_handler::ApiRequest;

use reqwest::{Client, Response, StatusCode};
use async_trait::async_trait;
pub struct ConversionRates
{
    url: String,
}

impl  ConversionRates {
    pub fn new(source: &String, api_key: &String, prefix: &String) -> Self
    {
        let url_ = format!("{prefix}/v6/{}/latest/{}", api_key, source);
        Self {
            url: url_
        }
    }
}

#[async_trait]
impl ApiRequest for ConversionRates 
{
    fn get_url(&self) -> String {
        self.url.clone()
    }

    async fn fetch(&self) -> Result<Option<FetchResult>, Box<dyn std::error::Error>> {
        let response: Response = Client::new()
            .get(self.get_url())
            .send()
            .await?;

        let status_code: StatusCode = response.status();
        if status_code.is_success() {
            let api_response: ApiResponse = response.json().await?;
            if let Some(supprted_codes) = api_response.conversion_rates {
                return Ok(Some(FetchResult::HashMapRates(supprted_codes)));
            }
            else {
                return Ok(None);
            }
        }
        else if status_code.is_client_error() {
            let api_response: ApiResponse = response.json().await?;
            let error =  self.handle_error(&api_response).await;
            match error{
                Ok(_) => Ok(None),
                Err(e) =>Err(e),
            }
        }
        else if status_code.is_server_error() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Server Error: {}", status_code))));
        }
        else {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Unexpected response status: {}", status_code))));

        }

    
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    use rust_decimal::Decimal;
    use std::collections::HashMap;
    use std::str::FromStr;


    #[tokio::test]
    async fn test_positive_fetch_standard_request() {
        //only first 5 values...
        let mock_server = MockServer::start().await;
        let body_mock_response = r#"
        {
            "result": "success",
            "documentation": "https://www.exchangerate-api.com/docs",
            "terms_of_use": "https://www.exchangerate-api.com/terms",
            "time_last_update_unix": 1711238401,
            "time_last_update_utc": "Sun, 24 Mar 2024 00:00:01 +0000",
            "time_next_update_unix": 1711324801,
            "time_next_update_utc": "Mon, 25 Mar 2024 00:00:01 +0000",
            "base_code": "USD",
            "conversion_rates": {
              "USD": 1,
              "AED": 3.6725,
              "AFN": 71.1809,
              "ALL": 94.7248,
              "AMD": 399.0437
            }
          }"#;

        let mock_response = ResponseTemplate::new(200).set_body_string(body_mock_response);

        Mock::given(method("GET"))
            .and(path("/v6/111111111111111111111111/latest/USD"))
            .respond_with(mock_response)
            .mount(&mock_server)
            .await;

        let source = "USD".to_string();
        let api_key = "111111111111111111111111".to_string();

        let prefix = format!("{}", mock_server.uri()).to_string();

        let standard_request = ConversionRates::new(&source, &api_key, &prefix);

        let mut expected_hashmap : HashMap<String, Decimal> = HashMap::new();
        expected_hashmap.insert("USD".to_string(), Decimal::from_str("1").unwrap());
        expected_hashmap.insert("AED".to_string(), Decimal::from_str("3.6725").unwrap());
        expected_hashmap.insert("AFN".to_string(), Decimal::from_str("71.1809").unwrap());
        expected_hashmap.insert("ALL".to_string(), Decimal::from_str("94.7248").unwrap());
        expected_hashmap.insert("AMD".to_string(), Decimal::from_str("399.0437").unwrap());

        match standard_request.fetch().await {
            Ok(Some(FetchResult::HashMapRates(convertion_result))) => {
                
                assert_eq!(convertion_result.len(), expected_hashmap.len());

                for (currency, rate) in &expected_hashmap {
                    assert!(convertion_result.contains_key(currency));
                    assert_eq!(&convertion_result[currency], rate);

                }
            },
            Ok(None) => {
                panic!("Incorrect Source/Target");
            },
            Err(e) => {
                panic!("Error fetching conversion rate: {}", e);
            },
            _ => (),
        }
    }

}