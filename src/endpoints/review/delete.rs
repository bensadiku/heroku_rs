//Anything related to DELETE requests for review app and it's properties goes here.
use super::ReviewApp;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Review App Delete
///
/// Delete an existing review app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-delete)
pub struct ReviewAppDelete {
    /// review_id is the unique identifier.
    pub review_id: String,
}

impl HerokuEndpoint<ReviewApp> for ReviewAppDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("review-apps/{}", self.review_id)
    }
}
