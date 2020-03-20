//Anything related to creating apps and it's properties goes here.
use super::App;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Create a new app.
/// No parameters required to create a new app.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-create
pub struct AppCreate {
    pub params: AppCreateParams,
}

/// Create a new app with parameters.
/// All three paramemters are optional.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-create-optional-parameters
#[derive(Serialize, Clone, Debug)]
pub struct AppCreateParams {
    pub name: Option<String>,
    pub region: Option<String>,
    pub stack: Option<String>,
}

impl HerokuEndpoint<App, (), AppCreateParams> for AppCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps")
    }
    fn body(&self) -> Option<AppCreateParams> {
        Some(self.params.clone())
    }
}

/// Enable ACM flag for an app
/// app_identifier is required to enable app acm.
/// app_identifier can be the app id or name.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-enable-acm
pub struct AppEnableAcm {
    pub app_identifier: String,
}

impl HerokuEndpoint<App> for AppEnableAcm {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/acm", self.app_identifier)
    }
}
