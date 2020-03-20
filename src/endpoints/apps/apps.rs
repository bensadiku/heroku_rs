use super::App;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Get info for existing app.
/// identifier can be the app id or app name.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-info
pub struct AppDetails {
    pub identifier: String,
}

impl HerokuEndpoint<App> for AppDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}", self.identifier)
    }
}

/// List existing apps.
/// No parameters required to get this list.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-list
pub struct AppList {}

impl HerokuEndpoint<Vec<App>> for AppList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps")
    }
}

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

/// Delete an existing app.
/// No parameters required to create a new app.
/// app_identifier can be the app id or app name.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-delete
pub struct AppDelete {
    pub app_identifier: String,
}

impl HerokuEndpoint<App> for AppDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}", self.app_identifier)
    }
}
