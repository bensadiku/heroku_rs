//Anything related to DELETE requests for spaces goes here.
use super::{Space, VPN};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Space Delete
///
/// Delete an existing space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-delete)
/// 
/// # Example:
///
/// SpaceDelete takes one required parameter, space_id, and returns the deleted [`Space`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&SpaceDelete::new("SPACE_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Space.html
pub struct SpaceDelete<'a> {
    /// unique space identifier, either space name or space id
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

/// Private Spaces VPN Destroy
///
/// Destroy existing VPN Connection
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#private-spaces-vpn-destroy)
/// 
/// # Example:
///
/// VPNDelete takes two required parameters, space_id and vpn_id, and returns the deleted [`VPN`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&VPNDelete::new("SPACE_ID", "VPN_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.VPN.html
pub struct VPNDelete<'a> {
    /// unique space identifier, either space name or space id
    pub space_id: &'a str,
    /// unique vpn identifier, either vpn connection name or vpn connection id
    pub vpn_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> VPNDelete<'a> {
    pub fn new(space_id: &'a str, vpn_id: &'a str) -> VPNDelete<'a> {
        VPNDelete { space_id, vpn_id }
    }
}

impl<'a> HerokuEndpoint<VPN> for VPNDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("spaces/{}/vpn-connections/{}", self.space_id, self.vpn_id)
    }
}
