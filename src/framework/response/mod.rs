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

#[derive(Deserialize, Debug)]
pub struct EmptyResponse {
   response: String
}

impl EmptyResponse {
    fn new() {
    }
}

impl ApiResult for EmptyResponse {

}

pub fn empty_response<T: ApiResult>(resp: reqwest::blocking::Response) -> ApiResponse<T> {
    let empty_resp = EmptyResponse::new();

    let parsed_resp: Result<T, reqwest::Error> = Ok(empty_resp);

    parsed_resp
}

/// Some endpoints return nothing.
impl ApiResult for () {}

pub trait ApiResult: DeserializeOwned + Debug {}
