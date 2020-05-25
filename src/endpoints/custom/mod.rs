use crate::framework::endpoint::{HerokuEndpoint, Method};
use crate::framework::response::ApiResult;
use serde::Serialize;
use serde_json::Value;

impl ApiResult for Value {}
impl ApiResult for Vec<Value> {}

/// CustomEndpoint
/// 
/// CustomEndpoint is way to query Heroku endpoints that have not been supported by the library yet.
/// 
/// See [more examples][examples]
///
/// [examples]: https://github.com/bensadiku/heroku_rs/blob/master/examples/src/custom_examples.rs
#[derive(Clone)]
pub struct CustomEndpoint<T>
where
    T: Serialize + Clone,
{
    /// the query you want
    /// e.g dynos/{dyno_id}
    pub query: String,
    /// the API method to use
    /// One of: Get, Post, Put, Delete, Patch
    pub method: Method,
    /// Parameters to pass to the Heroku API
    pub params: T,
}

#[cfg(feature = "builder")]
impl<T: Serialize + Clone> CustomEndpoint<T> {
    pub fn new(query: String, method: Method, params: T) -> CustomEndpoint<T> {
        CustomEndpoint {
            query,
            method,
            params,
        }
    }
}

impl<T> HerokuEndpoint<Value, (), T> for CustomEndpoint<T>
where
    T: Serialize + Clone,
{
    fn method(&self) -> Method {
        self.method
    }
    fn path(&self) -> String {
        format!("{}", self.query)
    }
    fn body(&self) -> Option<T> {
        Some(self.params.clone())
    }
}

/// CustomEndpointSimple
/// 
/// CustomEndpointSimple is way to query Heroku endpoints that have not been supported by the library yet.
/// 
/// # Example:
///
/// See [more examples][examples]
/// ```rust
/// use heroku_rs::prelude::*;
/// 
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// let app_id = "APP_ID_HERE";
/// let query = format!("{}{}", "apps/", app_id);
/// let method = Method::Get;
/// 
/// //This example does a GET request on `https://api.heroku.com/apps/APP_ID`
/// let response = api_client.request(&CustomEndpointSimple::new(query, method));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [examples]: https://github.com/bensadiku/heroku_rs/blob/master/examples/src/custom_examples.rs
#[derive(Clone)]
pub struct CustomEndpointSimple {
    /// the query you want
    /// e.g apps
    pub query: String,
    /// the API method to use
    /// One of: Get, Post, Put, Delete, Patch
    pub method: Method,
}

#[cfg(feature = "builder")]
impl CustomEndpointSimple {
    pub fn new(query: String, method: Method) -> CustomEndpointSimple {
        CustomEndpointSimple { query, method }
    }
}

impl HerokuEndpoint<Value> for CustomEndpointSimple {
    fn method(&self) -> Method {
        self.method
    }
    fn path(&self) -> String {
        format!("{}", self.query)
    }
}
