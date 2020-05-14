//Anything related to GET requests for domains and it's properties goes here.
use super::Domain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Domain Info
///
/// Info for existing domain.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain-info)
pub struct DomainDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// domain_id can be the domain hostname or id.
    pub domain_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> DomainDetails<'a> {
    pub fn new(app_id: &'a str, domain_id: &'a str) -> DomainDetails<'a> {
        DomainDetails { app_id, domain_id }
    }
}

impl<'a> HerokuEndpoint<Domain> for DomainDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/domains/{}", self.app_id, self.domain_id)
    }
}

/// Domain List
///
/// List existing domains.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain-list)
pub struct DomainList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> DomainList<'a> {
    pub fn new(app_id: &'a str) -> DomainList<'a> {
        DomainList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Domain>> for DomainList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/domains", self.app_id)
    }
}
