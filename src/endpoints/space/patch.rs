//Anything related to PATCH requests for spaces goes here.
use super::Space;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Space Update
///
/// Update an existing space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-update)
pub struct SpaceUpdate<'a> {
    /// unique space identifier, either space name or space if
    pub space_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: SpaceUpdateParams<'a>,
}

impl<'a> SpaceUpdate<'a> {
    pub fn new(space_id: &'a str) -> SpaceUpdate<'a> {
        SpaceUpdate {
            space_id,
            params: SpaceUpdateParams { name: None },
        }
    }

    pub fn name(&mut self, _name: &'a str) -> &mut Self {
        self.params.name = Some(_name);
        self
    }

    pub fn build(&self) -> SpaceUpdate<'a> {
        SpaceUpdate {
            space_id: self.space_id,
            params: SpaceUpdateParams {
                name: self.params.name,
            },
        }
    }
}

/// Update spaces with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-update-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct SpaceUpdateParams<'a> {
    /// unique name of space
    ///  pattern: `^[a-z0-9](?:[a-z0-9]
    pub name: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Space, (), SpaceUpdateParams<'a>> for SpaceUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("spaces/{}", self.space_id)
    }
    fn body(&self) -> Option<SpaceUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
