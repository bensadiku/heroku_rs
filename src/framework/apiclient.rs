//! This module contains the synchronous (blocking) API client.
use crate::framework::{
    endpoint::HerokuEndpoint,
    response::{ApiResponse, ApiResult},
};
use serde::Serialize;

/// Synchronous heroku client
pub trait HerokuApiClient {
    /// This returns a parsed Result<T, heroku_rs::framework::response::error::HerokuApiFailure>
    /// 
    /// Use this as the main method to interact with the Heroku API
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn HerokuEndpoint<ResultType, QueryType, BodyType>,
    ) -> ApiResponse<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize;

    /// This returns a Result<reqwest::blocking::Response, heroku_rs::framework::response::error::HerokuApiFailure>
    /// 
    /// This is primarily used for debugging and testing, but can be used if this works better for your use-case.
    fn request_raw<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn HerokuEndpoint<ResultType, QueryType, BodyType>,
    ) -> ApiResponse<reqwest::blocking::Response>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize;
}
