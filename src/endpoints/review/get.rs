//Anything related to GET requests for review app and it's properties goes here.
use super::ReviewApp;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Get Review App
///
/// Gets an existing review app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-get-review-app)
pub struct ReviewAppDetails {
    /// review_id is the unique identifier.
    pub review_id: String,
}

impl HerokuEndpoint<ReviewApp> for ReviewAppDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("review-apps/{}", self.review_id)
    }
}

/// Get Review App by App id
///
/// Get a review app using the associated app_id
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-get-review-app-by-app_id)
pub struct ReviewAppByAppDetails {
    /// app_id is the unique identifier, app name or app id.
    pub app_id: String,
}

impl HerokuEndpoint<ReviewApp> for ReviewAppByAppDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/review-app", self.app_id)
    }
}

/// Review App List by Pipeline id
///
/// List review apps for a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-list)
pub struct ReviewAppByPipelineList {
    /// app_id is the unique identifier, app name or app id.
    pub pipeline_id: String,
}

impl HerokuEndpoint<Vec<ReviewApp>> for ReviewAppByPipelineList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/review-apps", self.pipeline_id)
    }
}
