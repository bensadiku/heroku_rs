//Anything related to POST requests for domains and it's properties goes here.
use super::Domain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Domain Create
/// Create a new domain.
/// https://devcenter.heroku.com/articles/platform-api-reference#domain-create
pub struct DomainCreate {
    /// app_identifier can be the app name or id.
    pub app_identifier: String,
    /// The parameters to pass to the Heroku API
    pub params: DomainCreateParams,
}

/// Create domain with parameters.
/// https://devcenter.heroku.com/articles/platform-api-reference#domain-create-required-parameters
#[derive(Serialize, Clone, Debug)]
pub struct DomainCreateParams {
    /// full hostname
    pub hostname: String,
}

impl HerokuEndpoint<Domain, (), DomainCreateParams> for DomainCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/domains", self.app_identifier)
    }
    fn body(&self) -> Option<DomainCreateParams> {
        Some(self.params.clone())
    }
}
