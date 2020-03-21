use crate::framework::response::{ApiResult, Empty};
use crate::framework::ApiEnvironment;
use serde::Serialize;
use url::Url;

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

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
