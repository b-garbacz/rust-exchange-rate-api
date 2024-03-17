use std::collections::HashMap;
use reqwest::{Client, Response, StatusCode};
use rust_decimal::Decimal;
use serde::Deserialize;
use async_trait::async_trait;
use chrono::{TimeZone, Utc};

#[derive(Debug, Deserialize)]
pub struct ApiResponse 
{
    pub result: String,
    pub documentation: Option<String>,
    pub terms_of_use: Option<String>,
    pub time_last_update_unix: Option<i64>,
    pub time_last_update_utc: Option<String>,
    pub time_next_update_unix: Option<i64>,
    pub time_next_update_utc: Option<String>,
    pub base_code: Option<String>,
    pub conversion_rates: Option<HashMap<String, Decimal>>,
    pub supported_codes: Option<Vec<Vec<String>>>,
    pub conversion_result: Option<Decimal>,
    #[serde(rename = "error-type")]
    pub error_type: Option<String>,
}

#[derive(Debug)]
pub enum FetchResult {
    Decimal(Decimal),
    VecString(Vec<Vec<String>>),
    HashMapRates(HashMap<String, Decimal>)
}

#[async_trait]
pub trait ApiRequest
{
    async fn fetch(&self) -> Result<Option<FetchResult>, Box<dyn std::error::Error>>;
}

pub struct StandardRequest
{
    source: String,
    target: String,
    decimal_value: Decimal,
    api_key: String,
}

pub struct SupportedCodes
{
    api_key: String,
}

pub struct ConversionRates
{
    source: String,
    api_key: String,
}

impl StandardRequest
{
    pub fn new(source: &String, target: &String, decimal_value: Decimal, api_key: String) -> Self
    {

        StandardRequest {
            source: source.to_string(),
            target: target.to_string(),
            decimal_value,
            api_key,
        }
    }
    
    fn print_time(&self, api_response: &ApiResponse)
    {
    
        if let (Some(time_last_update_unix), Some(time_next_update_unix)) = (api_response.time_last_update_unix, api_response.time_next_update_unix) 
        {
            let time_last_update = Utc.timestamp_opt(time_last_update_unix, 0).unwrap();
            let time_next_update = Utc.timestamp_opt(time_next_update_unix, 0).unwrap();

            println!("Time of last update: {}", time_last_update);
            println!("Time of next update: {}", time_next_update);
        } 
        else
        {
            println!("Dates are not available");
        }
        
    }
}

impl SupportedCodes
{
    pub fn new(api_key: String) -> Self
    {
        SupportedCodes {
            api_key
        }
    }
}

impl  ConversionRates {
    pub fn new(source: String, api_key: String) -> Self
    {
        ConversionRates {
            source,
            api_key
        }
    }
}

#[async_trait]
impl ApiRequest for StandardRequest {
    async fn fetch(&self) -> Result<Option<FetchResult>, Box<dyn std::error::Error>> {
        let url = format!("https://v6.exchangerate-api.com/v6/{}/pair/{}/{}/{}", self.api_key, self.source, self.target, self.decimal_value);
        let response: Response = Client::new()
            .get(&url)
            .send()
            .await?;
        let status_code: StatusCode = response.status();
        if status_code.is_success()
        {   

            let api_response: ApiResponse = response.json().await?;
            self.print_time(&api_response);
            if let Some(convertion_result) = api_response.conversion_result{
                return Ok(Some(FetchResult::Decimal(convertion_result)));
            }
            else
            {
                return Ok(None);
            }
        }
        else if status_code.is_client_error() {
            let api_response: ApiResponse = response.json().await?;
            let error_message = if let Some(error_type) = api_response.error_type {
                match error_type.as_str()
                {
                    "unsupported-code" => "Unsupported currency code.",
                    "malformed-request" => "Malformed request.",
                    "invalid-key" => "Invalid API key.",
                    "inactive-account" => "Inactive account.",
                    "quota-reached" => "Request quota reached.",
                    _ => "Unknown error.",
                }
            } 
            else
            {
                "An error occurred, but no error type was provided."
            };
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)));
        } 
        else 
        {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Server Error")));
        }
    }
}

#[async_trait]
impl ApiRequest for SupportedCodes {
    async fn fetch(&self) -> Result<Option<FetchResult>, Box<dyn std::error::Error>>
    {
        let url = format!("https://v6.exchangerate-api.com/v6/{}/codes", self.api_key);
        let response: Response = Client::new().get(url).send().await?;
        let status_code: StatusCode = response.status();
        if status_code.is_success()
        {
            let api_response: ApiResponse = response.json().await?;
            if let Some(supprted_codes) = api_response.supported_codes{
                return Ok(Some(FetchResult::VecString(supprted_codes)));
            }
            else
            {
                return Ok(None);
            }

        }
        else if status_code.is_client_error()
        {
            let api_response: ApiResponse = response.json().await?;
            let error_message = if let Some(error_type) = api_response.error_type {
                match error_type.as_str()
                {
                    "invalid-key" => "Invalid API key.",
                    "inactive-account" => "Inactive account.",
                    "quota-reached" => "Request quota reached.",
                    _ => "Unknown error.",
                }
            } 
            else
            {
                "An error occurred, but no error type was provided."
            };
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)));
        }
        else
        {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Server Error")));
        }

    
    }
}

#[async_trait]
impl ApiRequest for ConversionRates 
{
    async fn fetch(&self) -> Result<Option<FetchResult>, Box<dyn std::error::Error>>
    {
        let url = format!("https://v6.exchangerate-api.com/v6/{}/latest/{}", self.api_key, self.source);
        let response: Response = Client::new().get(url).send().await?;
        let status_code: StatusCode = response.status();
        if status_code.is_success()
        {
            let api_response: ApiResponse = response.json().await?;
            if let Some(supprted_codes) = api_response.conversion_rates{
                return Ok(Some(FetchResult::HashMapRates(supprted_codes)));
            }
            else
            {
                return Ok(None);
            }

        }
        else if status_code.is_client_error()
        {
            let api_response: ApiResponse = response.json().await?;
            let error_message = if let Some(error_type) = api_response.error_type {
                match error_type.as_str()
                {
                    "unsupported-code" => "Unsupported currency code.",
                    "malformed-request" => "Malformed request.",
                    "invalid-key" => "Invalid API key.",
                    "inactive-account" => "Inactive account.",
                    "quota-reached" => "Request quota reached.",
                    _ => "Unknown error.",
                }
            } 
            else
            {
                "An error occurred, but no error type was provided."
            };
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)));
        }
        else
        {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Server Error")));
        }

    
    }
}
