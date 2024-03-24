use crate::models::{ApiResponse, FetchResult};
use crate::request_handler::ApiRequest;

use reqwest::{Client, Response, StatusCode};
use rust_decimal::Decimal;
use async_trait::async_trait;
use chrono::{TimeZone, Utc};

pub struct StandardRequest {
    url: String,
}

impl StandardRequest
{
    pub fn new(source: &String, target: &String, decimal_value: &Decimal, api_key: &String, prefix: &String) -> Self {
        let _url = format!("{prefix}/v6/{}/pair/{}/{}/{}", api_key, source, target, decimal_value);
        Self {
            url: _url,
        }
    }
    
    fn print_time(&self, api_response: &ApiResponse) {
    
        if let (Some(time_last_update_unix), Some(time_next_update_unix)) = (api_response.time_last_update_unix, api_response.time_next_update_unix) {
            let time_last_update = Utc.timestamp_opt(time_last_update_unix, 0).unwrap();
            let time_next_update = Utc.timestamp_opt(time_next_update_unix, 0).unwrap();

            println!("Time of last update: {}", time_last_update);
            println!("Time of next update: {}", time_next_update);
        } 
        else {
            println!("Dates are not available");
        }
        
    }

}

#[async_trait]
impl ApiRequest for StandardRequest {
    fn get_url(&self) -> String {
        self.url.clone()
    }

    async fn fetch(&self) -> Result<Option<FetchResult>, Box<dyn std::error::Error>> {
        println!("{}", self.url);
        let response: Response = Client::new()
            .get(self.get_url())
            .send()
            .await?;

        let status_code: StatusCode = response.status();

        if status_code.is_success() {
            let api_response: ApiResponse = response.json().await?;
            self.print_time(&api_response);
            if let Some(convertion_result) = api_response.conversion_result {
                return Ok(Some(FetchResult::Decimal(convertion_result)));
            }
            else {
                return Ok(None);
            }
        }
        else if status_code.is_client_error() {
            let api_response: ApiResponse = response.json().await?;
            let error =  self.handle_error(&api_response).await;
            match error {
                Ok(_) => Ok(None),
                Err(e) =>Err(e),
            }
        }
        else if status_code.is_server_error() {
            println!("{}",status_code);
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
    use std::str::FromStr;


    #[tokio::test]
    async fn test_positive_fetch_standard_request() {
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
            "target_code": "PLN",
            "conversion_rate": 3.9887,
            "conversion_result": 1994.35
        }"#;

        let mock_response = ResponseTemplate::new(200).set_body_string(body_mock_response);

        Mock::given(method("GET"))
            .and(path("/v6/111111111111111111111111/pair/USD/PLN/500"))
            .respond_with(mock_response)
            .mount(&mock_server)
            .await;

        let source = "USD".to_string();
        let target= "PLN".to_string();
        let decimal_value = Decimal::from_str("500").unwrap();
        let api_key = "111111111111111111111111".to_string();

        let prefix = format!("{}", mock_server.uri());

        let standard_request = StandardRequest::new(&source, &target, &decimal_value, &api_key, &prefix);

        match standard_request.fetch().await {
            Ok(Some(FetchResult::Decimal(convertion_result))) => {
                let expected_value = Decimal::from_str("1994.35").unwrap(); 
                assert_eq!(convertion_result, expected_value);
    
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

    #[tokio::test]
    async fn fetch_handles_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
        .and(path("/v6/111111111111111111111111/pair/USD/PLN/500"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

        let source = "USD".to_string();
        let target= "PLN".to_string();
        let decimal_value = Decimal::from_str("500").unwrap();
        let api_key = "111111111111111111111111".to_string();

        let prefix = format!("{}", mock_server.uri());

        let standard_request = StandardRequest::new(&source, &target, &decimal_value, &api_key, &prefix);
        
        match standard_request.fetch().await {
            Ok(Some(FetchResult::Decimal(_convertion_result))) => {
                panic!("The test failed: a server error was expected, but success was received.")
            },
            Ok(None) => {
                panic!("Incorrect Source/Target");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "Server Error: 500 Internal Server Error")
            },
            _ => (),
        }
        
    }
    
}
