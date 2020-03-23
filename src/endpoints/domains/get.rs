//Anything related to GET requests for domains and it's properties goes here.
use super::Domain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Domain Info
/// 
/// Info for existing domain.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain-info)
pub struct DomainDetails {
    /// app_id can be the app name or id.
    pub app_id: String,
    /// domain_id can be the domain hostname or id.
    pub domain_id: String,
}

impl HerokuEndpoint<Domain> for DomainDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/domains/{}",
            self.app_id, self.domain_id
        )
    }
}

/// Domain List
/// 
/// List existing domains.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain-list)
pub struct DomainList {
    /// app_id can be the app name or id.
    pub app_id: String,
}

impl HerokuEndpoint<Vec<Domain>> for DomainList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/domains", self.app_id)
    }
}
