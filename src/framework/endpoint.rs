use crate::framework::response::ApiResult;
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

pub trait HerokuEndpoint<ResultType = (), QueryType = (), BodyType = ()>
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
    fn content_type(&self) -> String {
        "application/json".to_owned()
    }
    fn version(&self) -> String {
        "application/vnd.heroku+json; version=3".to_owned()
    }
    fn agent(&self) -> String {
        "heroku_rs".to_owned()
    }
}
