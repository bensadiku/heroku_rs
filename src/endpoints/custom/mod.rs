use crate::framework::endpoint::{HerokuEndpoint, Method};
use crate::framework::response::ApiResult;
use serde::Serialize;
use serde_json::Value;

impl ApiResult for Value {}
impl ApiResult for Vec<Value> {}

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

/// A simple struct to query some endpoint in Heroku
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
