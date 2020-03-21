extern crate reqwest;
extern crate serde_json;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
mod error;

pub use error::*;
pub type ApiResponse<T> = Result<T, HerokuApiFailure>;

/// Match the response we just got from the API and return properly
pub fn match_response<T: ApiResult>(api_response: reqwest::blocking::Response) -> ApiResponse<T> {
    let api_status = api_response.status();

    if api_status.is_success() {
        let parsed_response: Result<T, reqwest::Error> = api_response.json();
        match parsed_response {
            Ok(response) => Ok(response),
            Err(e) => Err(HerokuApiFailure::Invalid(e)),
        }
    } else {
        let parsed: Result<HerokuApiError, reqwest::Error> = api_response.json();
        let errors = parsed.unwrap_or_default();
        Err(HerokuApiFailure::Error(api_status, errors))
    }
}

/// Some endpoints return empty objects or empty vectors.
impl ApiResult for Empty {}
impl ApiResult for Vec<Empty> {}

pub trait ApiResult: DeserializeOwned + Debug {}

/// This because Heroku returns a empty object in some responses.
#[derive(Deserialize, Serialize, Debug)]
pub struct Empty {}
