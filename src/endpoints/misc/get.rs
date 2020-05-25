//Anything related to GET requests for mixed endpoints goes here.
use super::{Ratelimit, Region, Stack};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Region Info
///
/// Info for existing region.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#region-info)
///
/// # Example:
///
/// RegionDetails takes one required parameter, region_id, and returns a [`Region`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let region_id = "6f2b2ec9-b087-4976-8ec9-5d2f62276aeb"; // Dublin - Ireland
/// let response = api_client.request(&RegionDetails::new(region_id));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Region.html
pub struct RegionDetails<'a> {
    /// region_id can be the region name or region id
    pub region_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// RegionList takes no required parameters, and returns a list of [`Regions`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&RegionList::new());
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Region.html
pub struct RegionList {}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// RatelimitDetails takes no required parameters, and returns the [`Ratelimit`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&RatelimitDetails::new());
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Ratelimit.html
pub struct RatelimitDetails {}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// StackList takes no required parameters, and returns a list of [`Stack`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&StackList::new());
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Stack.html
pub struct StackList {}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// StackDetails takes one required parameter, stack_id, and returns the [`Stack`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// 
/// let response = api_client.request(&StackDetails::new("STACK_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Stack.html
pub struct StackDetails<'a> {
    /// stack_id can be the stack name or stack id
    pub stack_id: &'a str,
}

#[cfg(feature = "builder")]
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
