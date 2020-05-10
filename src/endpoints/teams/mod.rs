use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{TeamDelete, TeamInvitationRevoke, TeamMemberDelete};
pub use get::{
    TeamAppDetails, TeamAppList, TeamAppPermissionList, TeamDetails, TeamFeatureDetails,
    TeamFeatureList, TeamInvitationDetails, TeamInvitationList, TeamInvoiceDetails,
    TeamInvoiceList, TeamList, TeamListByEA, TeamMemberAppsList, TeamMemberList,
    TeamPreferenceList,
};
pub use patch::{
    TeamAppTransfer, TeamAppTransferParams, TeamAppUpdateLocked, TeamAppUpdateLockedParams,
    TeamMemberUpdate, TeamMemberUpdateParams, TeamPreferenceUpdate, TeamPreferenceUpdateParams,
    TeamUpdate, TeamUpdateParams,
};
pub use post::{
    TeamAppCreate, TeamAppCreateParams, TeamCreate, TeamCreateByEA, TeamCreateByEAParams,
    TeamCreateParams, TeamInvitationAccept, TeamMemberCreate, TeamMemberCreateParams,
};
pub use put::{
    TeamInvitationCreate, TeamInvitationCreateParams, TeamMemberCreateorUpdate,
    TeamMemberCreateorUpdateParams,
};

impl ApiResult for Team {}
impl ApiResult for Vec<Team> {}

impl ApiResult for TeamApp {}
impl ApiResult for Vec<TeamApp> {}

impl ApiResult for Vec<TeamAppPermission> {}

impl ApiResult for TeamFeature {}
impl ApiResult for Vec<TeamFeature> {}

impl ApiResult for TeamInvitation {}
impl ApiResult for Vec<TeamInvitation> {}

impl ApiResult for TeamInvoice {}
impl ApiResult for Vec<TeamInvoice> {}

impl ApiResult for TeamMember {}
impl ApiResult for Vec<TeamMember> {}

impl ApiResult for TeamPreferences {}
impl ApiResult for Vec<TeamPreferences> {}

pub use team::Team;
pub use team_app::TeamApp;
pub use team_feature::TeamFeature;
pub use team_invitation::TeamInvitation;
pub use team_invoice::TeamInvoice;
pub use team_member::TeamMember;
pub use team_permission::TeamAppPermission;
pub use team_preferences::TeamPreferences;

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
    pub struct Team {
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

mod team_app {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Team App
    ///
    /// Stability: development
    ///
    /// A team app encapsulates the team specific functionality of Heroku apps.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#team-app)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct TeamApp {
        /// when app was archived
        pub archived_at: Option<DateTime<Utc>>,
        /// description from buildpack of app
        pub buildpack_provided_description: Option<String>,
        /// build stack
        pub build_stack: BuildStack,
        /// when app was created
        pub created_at: DateTime<Utc>,
        /// git repo URL of app
        /// pattern: ^https://git\.heroku\.com/[a-z][a-z0-9-]{2,29}\.git$
        pub git_url: String,
        /// unique identifier
        pub id: String,
        /// describes whether a Private Spaces app is externally routable or not
        pub internal_routing: Option<bool>,
        /// is the current member a collaborator on this app.
        pub joined: bool,
        /// are other team members forbidden from joining this app.
        pub locked: bool,
        /// maintenance status of app
        pub maintenance: bool,
        /// name of app
        /// pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
        pub name: String,
        /// team that owns this app
        pub team: Option<Team>,
        /// identity of app owner
        pub owner: Option<Owner>,
        /// A region represents a geographic location in which your application may run.
        pub region: Region,
        /// when app was released
        pub released_at: Option<DateTime<Utc>>,
        /// git repo size in bytes of app
        pub repo_size: Option<i64>,
        /// slug size in bytes of app
        pub slug_size: Option<i64>,
        /// identity of space
        pub space: Option<Space>,
        /// Stacks are the different application execution environments available in the Heroku platform.
        pub stack: Stack,
        /// when app was updated
        pub updated_at: DateTime<Utc>,
        /// web URL of app
        /// pattern: ^https?://[a-z][a-z0-9-]{3,30}\.herokuapp\.com/$
        pub web_url: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct BuildStack {
        /// identifier of stack
        pub id: String,
        /// unique name
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Team {
        /// unique name of team
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Owner {
        /// unique email address
        pub email: String,
        /// identifier of an account
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Region {
        /// unique identifier
        pub id: String,
        /// name of region
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Space {
        /// unique identifier of space
        pub id: String,
        /// unique name of space
        /// pattern: `^[a-z0-9](?:[a-z0-9]
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Stack {
        /// identifier of stack
        pub id: String,
        /// unique name
        pub name: String,
    }
}

mod team_permission {
    /// Team App Permission
    ///
    /// Stability: prototype
    ///
    /// A team app permission is a behavior that is assigned to a user in a team app.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#team-app-permission)
    ///
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct TeamAppPermission {
        /// The name of the app permission.
        pub name: String,
        /// A description of what the app permission allows.
        pub description: String,
    }
}

mod team_feature {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Team Feature
    ///
    /// Stability: development
    ///
    /// A team feature represents a feature enabled on a team account.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#team-feature)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct TeamFeature {
        /// when team feature was created
        pub created_at: DateTime<Utc>,
        /// description of team feature
        pub description: String,
        /// documentation URL of team feature
        pub doc_url: String,
        /// whether or not team feature has been enabled
        pub enabled: bool,
        /// e-mail to send feedback about the feature
        pub id: String,
        /// unique name of team feature
        pub name: String,
        /// state of team feature
        pub state: String,
        /// when team feature was updated
        pub updated_at: DateTime<Utc>,
        /// user readable feature name
        pub display_name: String,
        /// e-mail to send feedback about the feature
        pub feedback_email: String,
    }
}

mod team_invitation {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Team Invitation
    ///
    /// Stability: development
    ///
    /// A team invitation represents an invite to a team.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#team-invitation)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct TeamInvitation {
        /// when invitation was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of an invitation
        pub id: String,
        /// invited by
        pub invited_by: InvitedBy,
        /// team invited
        pub team: Team,
        /// role in the team
        ///  one of:"admin" or "collaborator" or "member" or "owner" or null
        pub role: Option<String>,
        /// when invitation was updated
        pub updated_at: DateTime<Utc>,
        /// account
        pub user: User,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct InvitedBy {
        /// unique email address
        pub email: String,
        /// identifier of an account
        pub id: String,
        /// full name of the account owner
        pub name: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Team {
        /// unique identifier of team
        pub id: String,
        /// unique name of team
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct User {
        /// unique email address
        pub email: String,
        /// identifier of an account
        pub id: String,
        /// full name of the account owner
        pub name: Option<String>,
    }
}

mod team_invoice {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Team Invoice
    ///
    /// Stability: development
    ///
    /// A Team Invoice is an itemized bill of goods for a team which includes pricing and charges.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#team-invoice)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TeamInvoice {
        /// total add-ons charges in on this invoice
        pub addons_total: i64,
        /// total database charges on this invoice
        pub database_total: i64,
        /// total charges on this invoice
        pub charges_total: i64,
        /// when invoice was created
        pub created_at: DateTime<Utc>,
        /// total credits on this invoice
        pub credits_total: i64,
        /// total amount of dyno units consumed across dyno types.
        pub dyno_units: f64,
        /// unique identifier of this invoice
        pub id: String,
        /// human readable invoice number
        pub number: i64,
        /// status of the invoice payment
        pub payment_status: String,
        /// the ending date that the invoice covers
        pub period_end: String,
        /// the starting date that this invoice covers
        pub period_start: String,
        /// total platform charges on this invoice
        pub platform_total: i64,
        /// payment status for this invoice (pending, successful, failed)
        pub state: i64,
        /// combined total of charges and credits on this invoice
        pub total: i64,
        /// when invoice was updated
        pub updated_at: DateTime<Utc>,
        /// The total amount of hours consumed across dyno types.
        pub weighted_dyno_hours: i64,
    }
}

mod team_member {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Team Member
    ///
    /// Stability: development
    ///
    /// A team member is an individual with access to a team.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#team-member)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TeamMember {
        /// when the membership record was created
        pub created_at: DateTime<Utc>,
        /// email address of the team member
        pub email: String,
        /// whether the user is federated and belongs to an Identity Provider
        pub federated: bool,
        /// unique identifier of the team member
        pub id: String,
        /// Identity Provider information the member is federated with
        pub identity_provider: Option<IdentityProvider>,
        /// role in the team
        /// one of:"admin" or "collaborator" or "member" or "owner" or null
        pub role: Option<String>,
        /// whether the Enterprise team member has two factor authentication enabled
        pub two_factor_authentication: bool,
        /// when the membership record was updated
        pub updated_at: DateTime<Utc>,
        /// account
        pub user: User,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct IdentityProvider {
        /// unique identifier of this identity provider
        pub id: String,
        /// name of the identity provider
        pub name: String,
        /// whether the identity_provider information is redacted or not
        pub redacted: bool,
        /// account owner
        pub owner: Owner,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Owner {
        /// unique identifier of the owner
        pub id: String,
        /// name of the owner
        pub name: String,
        /// type of the owner
        /// one of:"team" or "enterprise-account"
        #[serde(rename = "type")]
        pub type_field: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct User {
        /// unique email address
        pub email: String,
        /// identifier of an account
        pub id: String,
        /// full name of the account owner
        pub name: Option<String>,
    }
}

mod team_preferences {

    /// Team Preferences
    ///
    /// Stability: development
    ///
    /// Tracks a Team’s Preferences
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#team-preferences)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TeamPreferences {
        /// The default permission used when adding new members to the team
        ///  one of:"admin" or "member" or "viewer" or null
        #[serde(rename = "default-permission")]
        pub default_permission: Option<String>,
        /// Whether whitelisting rules should be applied to add-on installations
        #[serde(rename = "whitelisting-enabled")]
        pub whitelisting_enabled: Option<bool>,
    }
}
