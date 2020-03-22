//Anything related to DELETE requests for domains and it's properties goes here.
use super::Domain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Domain Delete
/// Delete an existing domain
/// https://devcenter.heroku.com/articles/platform-api-reference#domain-delete
pub struct DomainDelete {
    /// app_id can be the app name or id.
    pub app_id: String,
    /// domain_id can be the domain hostname or id.
    pub domain_id: String,
}

impl HerokuEndpoint<Domain> for DomainDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/domains/{}", self.app_id, self.domain_id)
    }
}
