extern crate reqwest;
extern crate serde_json;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub type ApiResponse<T> = Result<T, reqwest::Error>;

/// Match the response we just got from the API and return properly
pub fn match_response<T: ApiResult>(resp: reqwest::blocking::Response) -> ApiResponse<T> {
    let parsed_resp: Result<T, reqwest::Error> = resp.json();

    match parsed_resp {
        Ok(response) => Ok(response),
        Err(e) => Err(e),
    }
}

pub fn return_empty_response<T: ApiResult>(resp: reqwest::blocking::Response) -> ApiResponse<T> {

/// Some endpoints return nothing.
impl ApiResult for () {}

pub trait ApiResult: DeserializeOwned + Debug {}
