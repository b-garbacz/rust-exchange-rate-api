use crate::models::{ApiResponse, FetchResult};
use crate::request_handler::ApiRequest;

use reqwest::{Client, Response, StatusCode};
use async_trait::async_trait;

pub struct SupportedCodes {
    url: String
}

impl SupportedCodes {
    pub fn new(api_key: &String, prefix: &String) -> Self {
        let url_ = format!("{prefix}/v6/{}/codes", api_key);
        Self {
            url: url_,
        }
    }
}

#[async_trait]
impl ApiRequest for SupportedCodes {
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
            if let Some(supprted_codes) = api_response.supported_codes {
                return Ok(Some(FetchResult::VecString(supprted_codes)));
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
                Err(e) => Err(e),
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

