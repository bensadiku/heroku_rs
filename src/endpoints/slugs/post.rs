//Anything related to POST requests for slugs and it's variations goes here.
use super::Slug;

use crate::framework::endpoint::{HerokuEndpoint, Method};
use std::collections::HashMap;

/// Create Slug
///
/// Create a new slug. For more information please refer to ]Deploying Slugs using the Platform API.](https://devcenter.heroku.com/articles/platform-api-deploying-slugs)
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#slug-create)
pub struct SlugCreate {
    /// app_id is the unique app identifier.
    pub app_id: String,
    /// The parameters to pass to the Heroku API
    pub params: SlugCreateParams,
}

/// Create a new slug with parameters.
///
/// [See Heroku documentation for more information about these required parameters](https://devcenter.heroku.com/articles/platform-api-reference#slug-create-required-parameters)
///
/// [See Heroku documentation for more information about these optional parameters](https://devcenter.heroku.com/articles/platform-api-reference#slug-create-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct SlugCreateParams {
    /// hash mapping process type names to their respective command
    pub process_types: HashMap<String, String>,
    /// human-friendly description from buildpack of slug
    pub buildpack_provided_description: Option<String>,
    //// an optional checksum of the slug for verifying its integrity
    pub checksum: Option<String>,
    /// identification of the code with your version control system (eg: SHA of the git HEAD)
    pub commit: Option<String>,
    /// an optional description of the provided commit
    pub commit_description: Option<String>,
    /// unique name or identifier of stack
    pub stack: Option<String>,
}

impl HerokuEndpoint<Slug, (), SlugCreateParams> for SlugCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/slugs", self.app_id)
    }
    fn body(&self) -> Option<SlugCreateParams> {
        Some(self.params.clone())
    }
}
