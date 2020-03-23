use crate::framework::response::{ApiResult, Empty};
use crate::framework::ApiEnvironment;
use serde::Serialize;
use url::Url;

/// HTTP methods used on this crate.
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

/// Heroku Endpoint trait by default has a empty struct and void query types and body types
/// 
/// This trait is responsible for the majority of the functionality of this crate.
pub trait HerokuEndpoint<ResultType = Empty, QueryType = (), BodyType = ()>
where
    ResultType: ApiResult,
    QueryType: Serialize,
    BodyType: Serialize,
{
    fn method(&self) -> Method;
    fn path(&self) -> String;
    fn query(&self) -> Option<QueryType> {
        None
    }
    fn body(&self) -> Option<BodyType> {
        None
    }
    fn url(&self, environment: &ApiEnvironment) -> Url {
        Url::from(environment).join(&self.path()).unwrap()
    }
    fn content_type(&self) -> &str {
        "application/json"
    }
    fn version(&self) -> &str {
        "application/vnd.heroku+json; version=3"
    }
    fn agent(&self) -> &str {
        "heroku_rs"
    }
}
