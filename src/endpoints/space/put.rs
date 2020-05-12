//Anything related to PUT requests for spaces goes here.
use super::{InboundRuleset, OutboundRuleset};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Inbound Ruleset Create
///
/// Create a new inbound ruleset
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#inbound-ruleset-create)
pub struct InboundRulesetCreate<'a> {
    pub space_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: InboundRulesetCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> InboundRulesetCreate<'a> {
    pub fn new(space_id: &'a str) -> InboundRulesetCreate<'a> {
        InboundRulesetCreate {
            space_id,
            params: InboundRulesetCreateParams { rules: None },
        }
    }

    pub fn rule(&mut self, action: &'a str, source: &'a str) -> &mut Self {
        self.params.rules = Some(vec![(Rule { action, source })]);
        self
    }

    pub fn build(&self) -> InboundRulesetCreate<'a> {
        InboundRulesetCreate {
            space_id: self.space_id,
            params: InboundRulesetCreateParams {
                rules: self.params.rules.clone(),
            },
        }
    }
}

/// Create a new inbound rule set with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#inbound-ruleset-create-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct InboundRulesetCreateParams<'a> {
    /// rules to apply to the inbound ruleset
    pub rules: Option<Vec<Rule<'a>>>,
}
#[derive(Serialize, Clone, Debug)]
pub struct Rule<'a> {
    pub action: &'a str,
    pub source: &'a str,
}

impl<'a> HerokuEndpoint<InboundRuleset, (), InboundRulesetCreateParams<'a>>
    for InboundRulesetCreate<'a>
{
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!("spaces/{}/inbound-ruleset", self.space_id)
    }
    fn body(&self) -> Option<InboundRulesetCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Outbound Ruleset Create
///
/// Create a new outbound ruleset
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#outbound-ruleset-create)
pub struct OutboundRulesetCreate<'a> {
    pub space_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: OutboundRulesetCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> OutboundRulesetCreate<'a> {
    pub fn new(space_id: &'a str) -> OutboundRulesetCreate<'a> {
        OutboundRulesetCreate {
            space_id,
            params: OutboundRulesetCreateParams { rules: None },
        }
    }

    pub fn rule(
        &mut self,
        target: &'a str,
        protocol: &'a str,
        from_port: i64,
        to_port: i64,
    ) -> &mut Self {
        self.params.rules = Some(vec![
            (OutboundRule {
                target,
                protocol,
                from_port,
                to_port,
            }),
        ]);
        self
    }

    pub fn build(&self) -> OutboundRulesetCreate<'a> {
        OutboundRulesetCreate {
            space_id: self.space_id,
            params: OutboundRulesetCreateParams {
                rules: self.params.rules.clone(),
            },
        }
    }
}

/// Create a new inbound rule set with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#inbound-ruleset-create-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct OutboundRulesetCreateParams<'a> {
    /// rules to apply to the inbound ruleset
    pub rules: Option<Vec<OutboundRule<'a>>>,
}
#[derive(Serialize, Clone, Debug)]
pub struct OutboundRule<'a> {
    pub target: &'a str,
    pub protocol: &'a str,
    pub from_port: i64,
    pub to_port: i64,
}

impl<'a> HerokuEndpoint<OutboundRuleset, (), OutboundRulesetCreateParams<'a>>
    for OutboundRulesetCreate<'a>
{
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!("spaces/{}/inbound-ruleset", self.space_id)
    }
    fn body(&self) -> Option<OutboundRulesetCreateParams<'a>> {
        Some(self.params.clone())
    }
}
