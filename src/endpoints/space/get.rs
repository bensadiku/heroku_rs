//Anything related to GET requests for spaces goes here.
use super::{InboundRuleset, OutboundRuleset, Space, SpaceAccess, SpaceNAT, VPN};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Space Info
///
/// Info for existing space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-info)
/// 
/// # Example:
///
/// SpaceDetails takes one required parameter, space_id, and returns the [`Space`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&SpaceDetails::new("SPACE_ID"));
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
pub struct SpaceDetails<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// SpaceList takes no required parameters, and returns a list of [`Spaces`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&SpaceList::new());
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
pub struct SpaceList {}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// SpaceAccessDetails takes two required parameters, space_id and account_id, and returns the [`SpaceAccess`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&SpaceAccessDetails::new("SPACE_ID", "ACCOUNT_ID"));
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
/// [response]: ../struct.SpaceAccess.html
pub struct SpaceAccessDetails<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
    /// account_id can be the account email or account id
    pub account_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// SpaceAccessList takes one required parameter, space_id, and returns a list [`SpaceAccess`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&SpaceAccessList::new("SPACE_ID"));
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
/// [response]: ../struct.SpaceAccess.html
pub struct SpaceAccessList<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// SpaceNATDetails takes one required parameter, space_id, and returns the [`SpaceNAT`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&SpaceNATDetails::new("SPACE_ID"));
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
/// [response]: ../struct.SpaceNAT.html
pub struct SpaceNATDetails<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

#[cfg(feature = "builder")]
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

/// Inbound Ruleset Current
///
/// Current inbound ruleset for a space
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#inbound-ruleset-current)
/// 
/// # Example:
///
/// InboundRulesetCurrent takes one required parameter, space_id, and returns the [`InboundRuleset`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&InboundRulesetCurrent::new("SPACE_ID"));
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
/// [response]: ../struct.InboundRuleset.html
pub struct InboundRulesetCurrent<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> InboundRulesetCurrent<'a> {
    pub fn new(space_id: &'a str) -> InboundRulesetCurrent<'a> {
        InboundRulesetCurrent { space_id }
    }
}

impl<'a> HerokuEndpoint<InboundRuleset> for InboundRulesetCurrent<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("spaces/{}/inbound-ruleset", self.space_id)
    }
}

/// Inbound Ruleset Info
///
/// Info on an existing Inbound Ruleset
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#inbound-ruleset-info)
/// 
/// # Example:
///
/// InboundRulesetCurrent takes two required parameters, space_id and ruleset_id, and returns the current [`InboundRuleset`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&InboundRulesetCurrent::new("SPACE_ID"));
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
/// [response]: ../struct.InboundRuleset.html
pub struct InboundRulesetDetails<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
    /// unique inbound ruleset identifier
    pub ruleset_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> InboundRulesetDetails<'a> {
    pub fn new(space_id: &'a str, ruleset_id: &'a str) -> InboundRulesetDetails<'a> {
        InboundRulesetDetails {
            space_id,
            ruleset_id,
        }
    }
}

impl<'a> HerokuEndpoint<InboundRuleset> for InboundRulesetDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "spaces/{}/inbound-rulesets/{}",
            self.space_id, self.ruleset_id
        )
    }
}

/// Inbound Ruleset List
///
/// List all inbound rulesets for a space
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#inbound-ruleset-list)
///
/// # Example:
///
/// InboundRulesetList takes one required parameter, space_id, and returns a list of [`InboundRuleset`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&InboundRulesetList::new("SPACE_ID"));
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
/// [response]: ../struct.InboundRuleset.html
pub struct InboundRulesetList<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> InboundRulesetList<'a> {
    pub fn new(space_id: &'a str) -> InboundRulesetList<'a> {
        InboundRulesetList { space_id }
    }
}

impl<'a> HerokuEndpoint<Vec<InboundRuleset>> for InboundRulesetList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("spaces/{}/inbound-rulesets", self.space_id)
    }
}

/// Outbound Ruleset Current
///
/// Current outbound ruleset for a space
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#outbound-ruleset-current)
///
/// # Example:
///
/// OutboundRulesetCurrent takes one required parameter, space_id, and returns the current [`OutboundRuleset`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&OutboundRulesetCurrent::new("SPACE_ID"));
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
/// [response]: ../struct.OutboundRuleset.html
pub struct OutboundRulesetCurrent<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> OutboundRulesetCurrent<'a> {
    pub fn new(space_id: &'a str) -> OutboundRulesetCurrent<'a> {
        OutboundRulesetCurrent { space_id }
    }
}

impl<'a> HerokuEndpoint<OutboundRuleset> for OutboundRulesetCurrent<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("spaces/{}/outbound-ruleset", self.space_id)
    }
}

/// Outbound Ruleset Info
///
/// Info on an existing Outbound Ruleset
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#outbound-ruleset-info)
///
/// # Example:
///
/// OutboundRulesetDetails takes two required parameter, space_id and ruleset_id, and returns the [`OutboundRuleset`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&OutboundRulesetDetails::new("SPACE_ID", "RULESET_ID"));
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
/// [response]: ../struct.OutboundRuleset.html
pub struct OutboundRulesetDetails<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
    /// outbound ruleset unique identifier
    pub ruleset_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> OutboundRulesetDetails<'a> {
    pub fn new(space_id: &'a str, ruleset_id: &'a str) -> OutboundRulesetDetails<'a> {
        OutboundRulesetDetails {
            space_id,
            ruleset_id,
        }
    }
}

impl<'a> HerokuEndpoint<OutboundRuleset> for OutboundRulesetDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "spaces/{}/outbound-rulesets/{}",
            self.space_id, self.ruleset_id
        )
    }
}

/// Outbound Ruleset List
///
/// List all Outbound Rulesets for a space
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#outbound-ruleset-list)
///
/// # Example:
///
/// OutboundRulesetList takes one required parameter, space_id, and returns a list of [`OutboundRuleset`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&OutboundRulesetList::new("SPACE_ID"));
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
/// [response]: ../struct.OutboundRuleset.html
pub struct OutboundRulesetList<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> OutboundRulesetList<'a> {
    pub fn new(space_id: &'a str) -> OutboundRulesetList<'a> {
        OutboundRulesetList { space_id }
    }
}

impl<'a> HerokuEndpoint<Vec<OutboundRuleset>> for OutboundRulesetList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("spaces/{}/outbound-rulesets", self.space_id)
    }
}

/// Private Spaces VPN List
///
/// List VPN connections for a space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#private-spaces-vpn-list)
///
/// # Example:
///
/// VPNList takes one required parameter, space_id, and returns a list of [`VPN`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&VPNList::new("SPACE_ID"));
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
pub struct VPNList<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> VPNList<'a> {
    pub fn new(space_id: &'a str) -> VPNList<'a> {
        VPNList { space_id }
    }
}

impl<'a> HerokuEndpoint<Vec<VPN>> for VPNList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("spaces/{}/vpn-connections", self.space_id)
    }
}

/// Private Spaces VPN List
///
/// List VPN connections for a space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#private-spaces-vpn-list)
///
/// # Example:
///
/// VPNDetails takes one required parameter, space_id and vpn_id, and returns the [`VPN`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&VPNDetails::new("SPACE_ID", "VPN_ID"));
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
pub struct VPNDetails<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
    /// unique vpn identifier, either vpn connection name or vpn connection id
    pub vpn_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> VPNDetails<'a> {
    pub fn new(space_id: &'a str, vpn_id: &'a str) -> VPNDetails<'a> {
        VPNDetails { space_id, vpn_id }
    }
}

impl<'a> HerokuEndpoint<VPN> for VPNDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("spaces/{}/vpn-connections/{}", self.space_id, self.vpn_id)
    }
}
