use serde::{Serialize,Deserialize};
use rust_decimal::Decimal;
use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize)]
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