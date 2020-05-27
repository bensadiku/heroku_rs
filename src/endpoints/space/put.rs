//Anything related to PUT requests for spaces goes here.
use super::{InboundRuleset, OutboundRuleset};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Inbound Ruleset Create
///
/// Create a new inbound ruleset
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#inbound-ruleset-create)
///
/// # Example:
///
/// InboundRulesetCreate takes one required parameter, space_id, and returns the new [`InboundRuleset`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let space = &InboundRulesetCreate::new("SPACE_ID")
///     .rule("allow", "1.1.1.1/1")
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
/// [response]: ../struct.InboundRuleset.html
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

    /// # name: array of rules
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
///
/// # Example:
///
/// OutboundRulesetCreate takes one required parameter, space_id, and returns the new [`OutboundRuleset`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let target = "1.1.1.1/1";
/// let protocol = "tcp";
/// let to_port = 80;
/// let from_port = 80;
/// 
/// let space = &OutboundRulesetCreate::new("SPACE_ID")
///     .rule(target, protocol, from_port, to_port)
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
/// [response]: ../struct.OutboundRuleset.html
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
