use super::Dyno;

use crate::framework::endpoint::{HerokuEndpoint, Method};

use crate::endpoints::apps::App;

/// Get info for existing dyno.
/// app_identifier can be the app id or app name.
/// dyno_identifier can be the dyno id or dyno name
/// https://devcenter.heroku.com/articles/platform-api-reference#dyno-info
pub struct DynoDetails {
    pub app_identifier: String,
    pub identifier: String,
}

impl HerokuEndpoint<Dyno> for DynoDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos/{}", self.app_identifier, self.identifier)
    }
}

pub struct DynoList {
    pub app_identifier: String,
}

impl HerokuEndpoint<Vec<Dyno>> for DynoList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos", self.app_identifier)
    }
}

pub struct DynoRestart {
    pub app_identifier: String,
    pub identifier: String,
}

impl HerokuEndpoint<Dyno> for DynoRestart {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos/{}", self.app_identifier, self.identifier)
    }
}

pub struct DynoAllRestart {
    pub app_identifier: String,
}

impl HerokuEndpoint<> for DynoAllRestart {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos", self.app_identifier)
    }
}
