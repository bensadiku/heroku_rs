//! This module contains the synchronous (blocking) API client.
use crate::framework::{
    endpoint::HerokuEndpoint,
    response::{ApiResponse, ApiResult},
};
use serde::Serialize;

/// Synchronously sends requests to the Heroku API.
pub trait HerokuApiClient {
    /// Synchronously send a request to the Heroku API.
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn HerokuEndpoint<ResultType, QueryType, BodyType>,
    ) -> ApiResponse<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize;
}
