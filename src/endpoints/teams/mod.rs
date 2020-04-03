use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use get::{TeamDetails, TeamList};

impl ApiResult for Teams {}
impl ApiResult for Vec<Teams> {}

pub use team::Teams;

mod team {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Team
    ///
    /// Stability: development
    ///
    /// Teams allow you to manage access to a shared group of applications and other resources.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#team)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Teams {
        pub id: String,
        /// when the team was created
        pub created_at: DateTime<Utc>,
        /// whether charges incurred by the team are paid by credit card.
        pub credit_card_collections: bool,
        /// whether to use this team when none is specified
        pub default: bool,
        /// Entererprise account associated with the Team
        pub enterprise_account: Option<EnterpriseAccount>,
        /// Identity Provider associated with the Team
        pub identity_provider: Option<IdentityProvider>,
        /// upper limit of members allowed in a team.
        pub membership_limit: Option<i64>,
        /// unique name of team
        pub name: String,
        /// whether the team is provisioned licenses by salesforce.
        pub provisioned_licenses: bool,
        /// role in the team
        /// one of:"admin" or "collaborator" or "member" or "owner" or null(None)
        pub role: Option<String>,
        /// type of team.
        /// one of:"enterprise" or "team"
        #[serde(rename = "type")]
        pub type_field: String,
        /// when the team was updated
        pub updated_at: DateTime<Utc>,
    }

    /// Entererprise account associated with the Team
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct EnterpriseAccount {
        /// unique identifier of the enterprise account
        pub id: String,
        /// unique name of the enterprise account
        pub name: String,
    }

    /// Identity Provider associated with the Team
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct IdentityProvider {
        /// unique identifier of this identity provider
        pub id: String,
        /// user-friendly unique identifier for this identity provider
        pub slug: String,
    }
}
