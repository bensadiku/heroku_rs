use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use get::{
    InboundRulesetCurrent, InboundRulesetDetails, InboundRulesetList, OutboundRulesetCurrent,
    OutboundRulesetDetails, OutboundRulesetList, SpaceAccessDetails, SpaceAccessList, SpaceDetails,
    SpaceList, SpaceNATDetails,
};
pub use patch::{SpaceAccessUpdate, SpaceAccessUpdateParams, SpaceUpdate, SpaceUpdateParams};
pub use post::{SpaceCreate, SpaceCreateParams, SpaceTransferCreate, SpaceTransferCreateParams};
pub use put::{
    InboundRulesetCreate, InboundRulesetCreateParams, OutboundRulesetCreate,
    OutboundRulesetCreateParams,
};

impl ApiResult for Space {}
impl ApiResult for Vec<Space> {}

impl ApiResult for SpaceAccess {}
impl ApiResult for Vec<SpaceAccess> {}

impl ApiResult for SpaceNAT {}
impl ApiResult for Vec<SpaceNAT> {}

impl ApiResult for SpaceTransfer {}
impl ApiResult for Vec<SpaceTransfer> {}

impl ApiResult for InboundRuleset {}
impl ApiResult for Vec<InboundRuleset> {}

impl ApiResult for OutboundRuleset {}
impl ApiResult for Vec<OutboundRuleset> {}

pub use inbound_ruleset::InboundRuleset;
pub use outbound_ruleset::OutboundRuleset;
pub use space_access::SpaceAccess;
pub use space_nat::SpaceNAT;
pub use space_transfer::SpaceTransfer;
pub use spaces::Space;

mod spaces {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Space
    ///
    /// Stability: prototype
    ///
    /// A space is an isolated, highly available, secure app execution environments, running in the modern VPC substrate.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-1)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Space {
        /// when space was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of space
        pub id: String,
        /// unique name of space
        ///  pattern: `^[a-z0-9](?:[a-z0-9]
        pub name: String,
        /// organization this space is in
        pub organization: Organization,
        /// team this space is in
        pub team: Team,
        /// region of this space
        pub region: Region,
        /// true if this space has shield enabled
        pub shield: bool,
        /// availability of this space
        ///  one of:"allocating" or "allocated" or "deleting"
        pub state: String,
        /// when space was updated
        pub updated_at: DateTime<Utc>,
        /// The RFC-1918 CIDR the Private Space will use. It must be a /16 in 10.0.0.0/8, 172.16.0.0/12 or 192.168.0.0/16
        ///  default: "10.0.0.0/16"
        ///  pattern: `^((?:10
        pub cidr: String,
        /// The RFC-1918 CIDR that the Private Space will use for the Heroku-managed peering connection that’s automatically created when using Heroku Data add-ons. It must be between a /16 and a /20
        pub data_cidr: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Organization {
        /// unique name of team
        pub name: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Team {
        /// unique identifier of team
        pub id: String,
        /// unique name of team
        pub name: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Region {
        /// unique identifier
        pub id: String,
        /// name of region
        pub name: String,
    }
}

mod space_access {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Space Access
    ///
    /// Stability: prototype
    ///
    /// Space access represents the permissions a particular user has on a particular space.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-access)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct SpaceAccess {
        /// space object
        pub space: Space,
        /// when space was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of space
        pub id: String,
        /// permissions
        pub permissions: Vec<Permission>,
        /// when space was updated
        pub updated_at: DateTime<Utc>,
        /// account
        pub user: User,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Space {
        /// name of app
        ///  pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
        pub name: String,
        /// unique identifier
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Permission {
        pub description: String,
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct User {
        /// unique email address
        pub email: String,
        /// identifier of an account
        pub id: String,
    }
}

mod space_nat {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Space Network Address Translation
    ///
    /// Stability: prototype
    ///
    /// Network address translation (NAT) for stable outbound IP addresses from a space
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-network-address-translation)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct SpaceNAT {
        /// when network address translation for a space was created
        pub created_at: DateTime<Utc>,
        /// potential IPs from which outbound network traffic will originate
        pub sources: Vec<String>,
        /// availability of network address translation for a space
        ///  one of:"disabled" or "updating" or "enabled"
        pub state: String,
        /// when network address translation for a space was updated
        pub updated_at: DateTime<Utc>,
    }
}

mod space_transfer {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Space Transfer
    ///
    /// Stability: development
    ///
    /// Transfer spaces between enterprise teams with the same Enterprise Account.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-transfer)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct SpaceTransfer {
        pub created_at: DateTime<Utc>,
        pub id: String,
        pub name: String,
        pub organization: Organization,
        pub team: Team,
        pub region: Region,
        pub shield: bool,
        pub state: String,
        pub updated_at: DateTime<Utc>,
        pub cidr: String,
        pub data_cidr: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Organization {
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Team {
        pub id: String,
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Region {
        pub id: String,
        pub name: String,
    }
}

mod inbound_ruleset {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Inbound Ruleset
    ///
    /// Stability: prototype
    ///
    /// An inbound-ruleset is a collection of rules that specify what hosts can or cannot connect to an application.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#inbound-ruleset)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct InboundRuleset {
        /// unique identifier of an inbound-ruleset
        pub id: String,
        /// space
        pub space: Space,
        /// when inbound-ruleset was created
        pub created_at: DateTime<Utc>,
        /// rules
        pub rules: Option<Vec<Rule>>,
        /// unique email address
        pub created_by: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Space {
        /// unique identifier of space
        pub id: String,
        ///  pattern: `^[a-z0-9](?:[a-z0-9]
        pub name: String,
    }
    /// example: [{"action":"allow","source":"1.1.1.1/1"}]
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Rule {
        pub action: String,
        pub source: String,
    }
}

mod outbound_ruleset {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Outbound Ruleset
    ///
    /// Stability: prototype
    ///
    /// An outbound-ruleset is a collection of rules that specify what hosts Dynos are allowed to communicate with.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#outbound-ruleset)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct OutboundRuleset {
        /// unique identifier of an outbound-ruleset
        pub id: String,
        /// space object
        pub space: Space,
        /// when outbound-ruleset was created
        pub created_at: DateTime<Utc>,
        /// rules
        pub rules: Option<Vec<Rule>>,
        /// unique email address
        pub created_by: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Space {
        /// unique identifier of space
        pub id: String,
        /// unique name of space
        ///  pattern: `^[a-z0-9](?:[a-z0-9]
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Rule {
        pub target: String,
        pub from_port: i64,
        pub to_port: i64,
        pub protocol: String,
    }
}
