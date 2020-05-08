//Anything related to PUT requests for spaces goes here.
use super::InboundRuleset;

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
