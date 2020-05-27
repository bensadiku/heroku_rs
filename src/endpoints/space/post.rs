//Anything related to POST requests for spaces goes here.
use super::{Space, SpaceTransfer, VPN};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Space Create
///
/// Create a new space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-create)
///
/// # Example:
///
/// SpaceCreate takes three required parameters, unique space name and team name, and returns the new [`Space`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let space = &SpaceCreate::new("myspacename", "myteamname")
///     .cidr("123")
///     .data_cidr("10.2.0.0/16")
///     .region("6f2b2ec9-b087-4976-8ec9-5d2f62276aeb")
///     .shield(true)
///     .build();
/// let response = api_client.request(space);
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
pub struct SpaceCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: SpaceCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> SpaceCreate<'a> {
    pub fn new(name: &'a str, team: &'a str) -> SpaceCreate<'a> {
        SpaceCreate {
            params: SpaceCreateParams {
                name: name,
                team: team,
                cidr: None,
                data_cidr: None,
                region: None,
                shield: None,
            },
        }
    }

    /// # region: unique identifier or name of region
    pub fn region(&mut self, regions: &'a str) -> &mut Self {
        self.params.region = Some(regions);
        self
    }
    /// # cidr: The RFC-1918 CIDR the Private Space will use. It must be a /16 in 10.0.0.0/8, 172.16.0.0/12 or 192.168.0.0/16
    ///
    /// `default`: "10.0.0.0/16"
    ///
    /// `pattern`: `^((?:10
    pub fn cidr(&mut self, _cidr: &'a str) -> &mut Self {
        self.params.cidr = Some(_cidr);
        self
    }
    /// # data_cidr: The RFC-1918 CIDR that the Private Space will use for the Heroku-managed peering connection that’s automatically created when using Heroku Data add-ons. It must be between a /16 and a /20
    pub fn data_cidr(&mut self, _data_cidr: &'a str) -> &mut Self {
        self.params.data_cidr = Some(_data_cidr);
        self
    }
    /// # shield: true if this space has shield enabled
    pub fn shield(&mut self, _shield: bool) -> &mut Self {
        self.params.shield = Some(_shield);
        self
    }
    pub fn build(&self) -> SpaceCreate<'a> {
        SpaceCreate {
            params: SpaceCreateParams {
                name: self.params.name,
                team: self.params.team,
                cidr: self.params.cidr,
                data_cidr: self.params.data_cidr,
                region: self.params.region,
                shield: self.params.shield,
            },
        }
    }
}

/// Create a new space with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct SpaceCreateParams<'a> {
    /// unique name of space
    ///  pattern: `^[a-z0-9](?:[a-z0-9]
    pub name: &'a str,
    /// unique name of team
    pub team: &'a str,
    /// The RFC-1918 CIDR the Private Space will use. It must be a /16 in 10.0.0.0/8, 172.16.0.0/12 or 192.168.0.0/16
    ///  default: "10.0.0.0/16"
    ///  pattern: `^((?:10
    pub cidr: Option<&'a str>,
    /// The RFC-1918 CIDR that the Private Space will use for the Heroku-managed peering connection that’s automatically created when using Heroku Data add-ons.
    /// It must be between a /16 and a /20
    pub data_cidr: Option<&'a str>,
    /// unique identifier or name of region
    pub region: Option<&'a str>,
    /// true if this space has shield enabled
    pub shield: Option<bool>,
}

impl<'a> HerokuEndpoint<Space, (), SpaceCreateParams<'a>> for SpaceCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("spaces")
    }
    fn body(&self) -> Option<SpaceCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Space Transfer
///
/// Transfer space between enterprise teams
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-transfer-transfer)
///
/// # Example:
///
/// SpaceTransferCreate takes two required parameters, space_id and new_owner, and returns the new [`SpaceTransfer`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let new_owner_id = "123";
/// let space_id = "123";
/// let space = &SpaceTransferCreate::new(space_id, new_owner_id);
/// let response = api_client.request(space);
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
/// [response]: ../struct.SpaceTransfer.html
pub struct SpaceTransferCreate<'a> {
    /// unique space identifier, either space name or space id
    pub space_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: SpaceTransferCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> SpaceTransferCreate<'a> {
    pub fn new(space_id: &'a str, new_owner: &'a str) -> SpaceTransferCreate<'a> {
        SpaceTransferCreate {
            space_id,
            params: SpaceTransferCreateParams { new_owner },
        }
    }
}

/// Create a new space transfer with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-transfer-transfer-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct SpaceTransferCreateParams<'a> {
    /// unique name of team
    pub new_owner: &'a str,
}

impl<'a> HerokuEndpoint<SpaceTransfer, (), SpaceTransferCreateParams<'a>>
    for SpaceTransferCreate<'a>
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("spaces/{}/transfer", self.space_id)
    }
    fn body(&self) -> Option<SpaceTransferCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Private Spaces VPN Create
///
/// Create a new VPN connection in a private space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#private-spaces-vpn-create)
///
/// # Example:
///
/// VPNCreate takes four required parameters, space_id and new_owner, and returns the new [`VPN`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let space = &VPNCreate::new("SPACE_ID", "office", "35.161.69.30", vec!["172.16.0.0/16"]);
/// let response = api_client.request(space);
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
pub struct VPNCreate<'a> {
    /// unique space identifier, either space name or space id
    pub space_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: VPNCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> VPNCreate<'a> {
    pub fn new(
        space_id: &'a str,
        name: &'a str,
        public_ip: &'a str,
        routable_cidrs: Vec<&'a str>,
    ) -> VPNCreate<'a> {
        VPNCreate {
            space_id,
            params: VPNCreateParams {
                name,
                public_ip,
                routable_cidrs,
            },
        }
    }
}

/// Create a new private space vpn with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#private-spaces-vpn-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct VPNCreateParams<'a> {
    /// VPN Name
    pub name: &'a str,
    /// Public IP of VPN customer gateway
    pub public_ip: &'a str,
    /// A list of Routable CIDRs of VPN
    pub routable_cidrs: Vec<&'a str>,
}

impl<'a> HerokuEndpoint<VPN, (), VPNCreateParams<'a>> for VPNCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("spaces/{}/vpn-connections", self.space_id)
    }
    fn body(&self) -> Option<VPNCreateParams<'a>> {
        Some(self.params.clone())
    }
}
