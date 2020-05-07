//Anything related to GET requests for spaces goes here.
use super::{Space, SpaceAccess, SpaceNAT};

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

/// Space Access Info
///
/// List permissions for a given user on a given space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-access-info)
pub struct SpaceAccessDetails<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
    /// account_id can be the account email or account id
    pub account_id: &'a str,
}

impl<'a> SpaceAccessDetails<'a> {
    pub fn new(space_id: &'a str, account_id: &'a str) -> SpaceAccessDetails<'a> {
        SpaceAccessDetails {
            space_id,
            account_id,
        }
    }
}

impl<'a> HerokuEndpoint<SpaceAccess> for SpaceAccessDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("spaces/{}/members/{}", self.space_id, self.account_id)
    }
}

/// Space Access List
///
/// List all users and their permissions on a space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-access-list)
pub struct SpaceAccessList<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

impl<'a> SpaceAccessList<'a> {
    pub fn new(space_id: &'a str) -> SpaceAccessList<'a> {
        SpaceAccessList { space_id }
    }
}

impl<'a> HerokuEndpoint<Vec<SpaceAccess>> for SpaceAccessList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("spaces/{}/members", self.space_id)
    }
}


/// Space Network Address Translation Info
///
/// Current state of network address translation for a space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-network-address-translation-info)
pub struct SpaceNATDetails<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

impl<'a> SpaceNATDetails<'a> {
    pub fn new(space_id: &'a str) -> SpaceNATDetails<'a> {
        SpaceNATDetails { space_id }
    }
}

impl<'a> HerokuEndpoint<SpaceNAT> for SpaceNATDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("spaces/{}/nat", self.space_id)
    }
}
