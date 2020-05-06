//Anything related to GET requests for spaces goes here.
use super::Space;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Space Info
///
/// Info for existing space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-info)
pub struct SpaceDetails<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

impl<'a> SpaceDetails<'a> {
    pub fn new(space_id: &'a str) -> SpaceDetails<'a> {
        SpaceDetails { space_id }
    }
}

impl<'a> HerokuEndpoint<Space> for SpaceDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("spaces/{}", self.space_id)
    }
}

/// Space List
///
/// List existing spaces.   
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-space-list-1)
pub struct SpaceList {}

impl SpaceList {
    pub fn new() -> SpaceList {
        SpaceList {}
    }
}

impl HerokuEndpoint<Vec<Space>> for SpaceList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("spaces")
    }
}
