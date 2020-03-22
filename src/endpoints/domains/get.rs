//Anything related to GET requests for domains and it's properties goes here.
use super::Domain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Domain Info
/// Info for existing domain.
/// https://devcenter.heroku.com/articles/platform-api-reference#domain-info
pub struct DomainDetails {
    /// app_identifier can be the app name or id.
    pub app_identifier: String,
    /// domain_identifier can be the domain hostname or id.
    pub domain_identifier: String,
}

impl HerokuEndpoint<Domain> for DomainDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/domains/{}",
            self.app_identifier, self.domain_identifier
        )
    }
}

/// Domain List
/// List existing domains.
/// https://devcenter.heroku.com/articles/platform-api-reference#domain-list
pub struct DomainList {
    /// app_identifier can be the app name or id.
    pub app_identifier: String,
}

impl HerokuEndpoint<Vec<Domain>> for DomainList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/domains", self.app_identifier)
    }
}
