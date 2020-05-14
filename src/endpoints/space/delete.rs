//Anything related to DELETE requests for spaces goes here.
use super::Space;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Space Delete
///
/// Delete an existing space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-delete)
pub struct SpaceDelete<'a> {
    /// unique space identifier, either space name or space if
    pub space_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> SpaceDelete<'a> {
    pub fn new(space_id: &'a str) -> SpaceDelete<'a> {
        SpaceDelete { space_id }
    }
}

impl<'a> HerokuEndpoint<Space> for SpaceDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("spaces/{}", self.space_id)
    }
}
