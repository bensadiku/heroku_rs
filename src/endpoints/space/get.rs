//Anything related to GET requests for spaces goes here.
use super::{InboundRuleset, OutboundRuleset, Space, SpaceAccess, SpaceNAT};

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

/// Inbound Ruleset Current
///
/// Current inbound ruleset for a space
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#inbound-ruleset-current)
pub struct InboundRulesetCurrent<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

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
pub struct InboundRulesetDetails<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
    /// unique inbound ruleset identifier
    pub ruleset_id: &'a str,
}

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
pub struct InboundRulesetList<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

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
pub struct OutboundRulesetCurrent<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

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
pub struct OutboundRulesetDetails<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
    /// outbound ruleset unique identifier
    pub ruleset_id: &'a str,
}

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
pub struct OutboundRulesetList<'a> {
    /// space_id can be the space name or space id
    pub space_id: &'a str,
}

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
