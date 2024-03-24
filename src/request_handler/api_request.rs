use async_trait::async_trait;
use crate::models::{ApiResponse, FetchResult};

#[async_trait]
pub trait ApiRequest
{
    async fn fetch(&self) -> Result<Option<FetchResult>, Box<dyn std::error::Error>>;
    fn get_url(&self) -> String;
    async fn handle_error(&self, api_response: &ApiResponse) -> Result<(), Box<dyn std::error::Error>>
    {
        let message_error =  match api_response.error_type.as_deref() {
            Some("unsupported-code") => "Unsupported currency code.",
            Some("malformed-request") => "Malformed request.",
            Some("invalid-key") => "Invalid API key.",
            Some("inactive-account") => "Account is inactive.",
            Some("quota-reached") => "Request quota reached.",
            _ => "An error occurred, but no specific error type was provided.",
        };
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, message_error)));
    }
}