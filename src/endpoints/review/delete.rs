//Anything related to DELETE requests for review app and it's properties goes here.
use super::{ReviewApp, ReviewAppConfig};

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

impl ReviewAppDelete {
    pub fn new(review_id: String) -> ReviewAppDelete {
        ReviewAppDelete { review_id }
    }
}

impl HerokuEndpoint<ReviewApp> for ReviewAppDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("review-apps/{}", self.review_id)
    }
}

/// Review App Configuration Delete
///
/// Disable review apps for a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-configuration-delete)
pub struct ReviewAppConfigDelete<'a> {
    /// pipeline_id is the unique identifier.
    pub pipeline_id: &'a str,
}

impl<'a> ReviewAppConfigDelete<'a> {
    pub fn new(pipeline_id: &'a str) -> ReviewAppConfigDelete<'a> {
        ReviewAppConfigDelete { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<ReviewAppConfig> for ReviewAppConfigDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("pipelines/{}/review-app-config", self.pipeline_id)
    }
}
