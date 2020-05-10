//Anything related to GET requests for mixed endpoints goes here.
use super::{Ratelimit, Region, Stack};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Region Info
///
/// Info for existing region.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#region-info)
pub struct RegionDetails<'a> {
    /// region_id can be the region name or region id
    pub region_id: &'a str,
}

impl<'a> RegionDetails<'a> {
    pub fn new(region_id: &'a str) -> RegionDetails<'a> {
        RegionDetails { region_id }
    }
}

impl<'a> HerokuEndpoint<Region> for RegionDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("regions/{}", self.region_id)
    }
}

/// Region List
///
/// List existing regions.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#region-list)
pub struct RegionList {}

impl RegionList {
    pub fn new() -> RegionList {
        RegionList {}
    }
}

impl HerokuEndpoint<Vec<Region>> for RegionList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("regions")
    }
}

/// Rate Limit Info
///
/// Info for rate limits.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#rate-limit-info)
pub struct RatelimitDetails {}

impl RatelimitDetails {
    pub fn new() -> RatelimitDetails {
        RatelimitDetails {}
    }
}

impl HerokuEndpoint<Ratelimit> for RatelimitDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/rate-limits")
    }
}

/// Stack List
///
/// List available stacks.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#stack-list)
pub struct StackList {}

impl StackList {
    pub fn new() -> StackList {
        StackList {}
    }
}

impl HerokuEndpoint<Vec<Stack>> for StackList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("stacks")
    }
}

/// Stack Info
///
/// Info about a specific stack.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#stack-info)
pub struct StackDetails<'a> {
    /// stack_id can be the stack name or stack id
    pub stack_id: &'a str,
}

impl<'a> StackDetails<'a> {
    pub fn new(stack_id: &'a str) -> StackDetails<'a> {
        StackDetails { stack_id }
    }
}

impl<'a> HerokuEndpoint<Stack> for StackDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("stacks/{}", self.stack_id)
    }
}
