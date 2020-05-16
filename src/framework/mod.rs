//! Module for for authentication, api clients and response parsing.

pub mod apiclient;
pub mod auth;
pub mod endpoint;
mod reqwest_utils;
pub mod response;

use crate::framework::{apiclient::HerokuApiClient, auth::AuthClient, response::match_response};
use failure::Fallible;
use reqwest_utils::match_reqwest_method;
use serde::Serialize;
use std::time::Duration;

#[derive(Debug)]
pub enum ApiEnvironment {
    Production,
    Custom(url::Url),
}

impl<'a> From<&'a ApiEnvironment> for url::Url {
    fn from(environment: &ApiEnvironment) -> Self {
        match environment {
            ApiEnvironment::Production => url::Url::parse("https://api.heroku.com/").unwrap(),
            ApiEnvironment::Custom(url) => url.clone(),
        }
    }
}

/// The client used to make requests to Heroku.
///
/// This struct contains the synchronous client.
pub struct HttpApiClient {
    /// The base endpoint to target. By default will be heroku
    environment: ApiEnvironment,
    /// The authentication credential
    credentials: auth::Credentials,
    /// The blocking client
    http_client: reqwest::blocking::Client,
}

/// Configuration for the API client. Allows users to customize its behaviour.
pub struct HttpApiClientConfig {
    /// The maximum time limit for an API request. If a request takes longer than this, it will be cancelled.
    /// By default this duration will be 30 seconds because that's the max timeout before Heroku terminates the request
    pub http_timeout: Duration,
    /// A default set of HTTP headers which will be sent with each API request.
    pub default_headers: http::HeaderMap,
}

impl Default for HttpApiClientConfig {
    fn default() -> Self {
        HttpApiClientConfig {
            http_timeout: Duration::from_secs(30),
            default_headers: http::HeaderMap::default(),
        }
    }
}

impl HttpApiClient {
    /// # Example 1:
    /// Creating a simple client with the defaults. This has the production Heroku endpoint, 30 seconds timeout and the standard api key authentication.
    /// ```rust
    /// use heroku_rs::prelude::*;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///    let api_client = HttpApiClient::create("API_KEY")?;
    ///
    ///    // you can start making requests here
    ///
    ///    Ok(())
    /// }
    /// ```
    pub fn create(token: &str) -> Fallible<HttpApiClient> {
        let credentials: auth::Credentials = auth::Credentials::UserAuthToken {
            token: String::from(token),
        };
        let config: HttpApiClientConfig = HttpApiClientConfig::default();
        let environment: ApiEnvironment = ApiEnvironment::Production;
        let http_client = reqwest::blocking::Client::builder()
            .timeout(config.http_timeout)
            .default_headers(config.default_headers)
            .build()?;

        Ok(HttpApiClient {
            environment,
            credentials,
            http_client,
        })
    }

    /// # Example 2:
    /// Creating a custom client in which you can specify the custom endpoint, timeouts and custom credentials.
    ///
    /// This example is using a API key to authenticate, 10 second timeouts, and `https://api.custom-somewhere.com/` base endpoint.
    ///
    /// This is primarily used for testing / developement.
    /// ```rust
    /// use heroku_rs::prelude::*;
    /// use std::time::Duration;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///    let credentials = Credentials::UserAuthToken {
    ///         token: String::from("API_KEY"),
    ///    };
    ///
    ///   let api_client = HttpApiClient::new(
    ///    credentials,
    ///    HttpApiClientConfig {
    ///        http_timeout: Duration::from_secs(10),
    ///        default_headers: http::HeaderMap::default(),
    ///    },
    ///    ApiEnvironment::Custom(url::Url::parse("https://api.custom-somewhere.com/").unwrap()))?;
    ///
    ///    // you can start making requests here with api_client
    ///
    ///    Ok(())
    /// }
    ///
    /// ```
    pub fn new(
        credentials: auth::Credentials,
        config: HttpApiClientConfig,
        environment: ApiEnvironment,
    ) -> Fallible<HttpApiClient> {
        let http_client = reqwest::blocking::Client::builder()
            .timeout(config.http_timeout)
            .default_headers(config.default_headers)
            .build()?;

        Ok(HttpApiClient {
            environment,
            credentials,
            http_client,
        })
    }
}

impl<'a> HerokuApiClient for HttpApiClient {
    /// Synchronously send a request to the Heroku API.
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn endpoint::HerokuEndpoint<ResultType, QueryType, BodyType>,
    ) -> response::ApiResponse<ResultType>
    where
        ResultType: response::ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        // Build the request
        let mut request = self
            .http_client
            .request(
                match_reqwest_method(endpoint.method()),
                endpoint.url(&self.environment),
            )
            .query(&endpoint.query());

        if let Some(body) = endpoint.body() {
            request = request.body(serde_json::to_string(&body).unwrap());
            request = request.header(reqwest::header::CONTENT_TYPE, endpoint.content_type());
        }

        request = request.header(reqwest::header::ACCEPT, endpoint.version());
        request = request.header(reqwest::header::USER_AGENT, endpoint.agent());
        request = request.auth(&self.credentials);

        let response = request.send()?;

        match_response(response)
    }

    fn request_raw<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn endpoint::HerokuEndpoint<ResultType, QueryType, BodyType>,
    ) -> response::ApiResponse<reqwest::blocking::Response>
    where
        ResultType: response::ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        // Build the raw request
        let mut request = self
            .http_client
            .request(
                match_reqwest_method(endpoint.method()),
                endpoint.url(&self.environment),
            )
            .query(&endpoint.query());

        // Add body if one was passed
        if let Some(body) = endpoint.body() {
            request = request.body(serde_json::to_string(&body).unwrap());
            request = request.header(reqwest::header::CONTENT_TYPE, endpoint.content_type());
        }

        request = request.header(reqwest::header::ACCEPT, endpoint.version());
        request = request.header(reqwest::header::USER_AGENT, endpoint.agent());
        request = request.auth(&self.credentials);

        let response = request.send()?;
        Ok(response)
    }
}
