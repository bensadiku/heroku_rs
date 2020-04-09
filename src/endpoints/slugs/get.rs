//Anything related to GET requests for slugs and it's variations goes here.
use super::Slug;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Slug Info
///
/// Info for existing slug.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#slug-info)
pub struct SlugDetails {
    /// unique app identifier.
    pub app_id: String,
    /// unique slug indentifier.
    pub slug_id: String,
}

impl SlugDetails {
    pub fn new(app_id: String, slug_id: String) -> SlugDetails {
        SlugDetails { app_id, slug_id }
    }
}

impl HerokuEndpoint<Slug> for SlugDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/slugs/{}", self.app_id, self.slug_id)
    }
}
