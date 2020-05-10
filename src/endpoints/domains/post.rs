//Anything related to POST requests for domains and it's properties goes here.
use super::Domain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Domain Create
///
/// Create a new domain.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain-create)
pub struct DomainCreate<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: DomainCreateParams<'a>,
}

impl<'a> DomainCreate<'a> {
    pub fn new(app_id: &'a str, hostname: &'a str) -> DomainCreate<'a> {
        DomainCreate {
            app_id,
            params: DomainCreateParams { hostname },
        }
    }
}

/// Create domain with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct DomainCreateParams<'a> {
    /// full hostname
    pub hostname: &'a str,
}

impl<'a> HerokuEndpoint<Domain, (), DomainCreateParams<'a>> for DomainCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/domains", self.app_id)
    }
    fn body(&self) -> Option<DomainCreateParams<'a>> {
        Some(self.params.clone())
    }
}
