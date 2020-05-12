//Anything related to POST requests for mixed endpoints goes here.
use super::SourceBlob;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Source Create
///
/// Create URLs for uploading and downloading source.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#source-create)
pub struct SourceCreate {}

#[cfg(feature = "builder")]
impl SourceCreate {
    pub fn new() -> SourceCreate {
        SourceCreate {}
    }
}

impl HerokuEndpoint<SourceBlob> for SourceCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("sources")
    }
}
