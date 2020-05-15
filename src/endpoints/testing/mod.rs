use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use get::{TestCaseList, TestNodeList, TestRunDetails, TestRunDetailsByPipeline, TestRunList};
pub use patch::{TestRunUpdate, TestRunUpdateParams};
pub use post::{TestRunCreate, TestRunCreateParams};

impl ApiResult for TestCase {}
impl ApiResult for Vec<TestCase> {}

impl ApiResult for TestNode {}
impl ApiResult for Vec<TestNode> {}

impl ApiResult for TestRun {}
impl ApiResult for Vec<TestRun> {}

pub use test_case::TestCase;
pub use test_node::TestNode;
pub use test_run::TestRun;

mod test_case {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Test Case
    ///
    /// Stability: prototype
    ///
    /// A single test case belonging to a test run
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-case)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TestCase {
        /// unique identifier of a test case
        pub id: String,
        /// when test case was created
        pub created_at: DateTime<Utc>,
        /// when test case was updated
        pub updated_at: DateTime<Utc>,
        /// description of the test case
        pub description: String,
        /// meta information about the test case
        pub diagnostic: String,
        /// special note about the test case e.g. skipped, todo
        pub directive: String,
        /// whether the test case was successful
        pub passed: bool,
        /// the test number
        pub number: i64,
        /// Test node
        pub test_node: TestNode,
        /// Test run
        pub test_run: TestRun,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TestNode {
        /// unique identifier of a test node
        pub id: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TestRun {
        /// unique identifier of a test run
        pub id: String,
    }
}

mod test_node {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Test Node
    ///
    /// Stability: prototype
    ///
    /// A single test node belonging to a test run
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-node)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TestNode {
        /// when test node was created
        pub created_at: DateTime<Utc>,
        /// the dyno which belongs to this test node
        pub dyno: Option<Dyno>,
        /// the status of the test run when the error occured
        pub error_status: Option<String>,
        /// the exit code of the test script
        pub exit_code: Option<i64>,
        /// unique identifier of a test node
        pub id: String,
        /// the index of the test node
        pub index: i64,
        /// human friendly message indicating reason for an error
        pub message: Option<String>,
        /// the streaming output for the test node
        pub output_stream_url: String,
        /// pipeline
        pub pipeline: Pipeline,
        /// the streaming test setup output for the test node
        pub setup_stream_url: String,
        /// current state of the test run
        ///  one of:"pending" or "cancelled" or "creating" or "building" or "running" or "succeeded" or "failed" or "errored" or "debugging"
        pub status: String,
        /// when test node was updated
        pub updated_at: DateTime<Utc>,
        /// test run
        pub test_run: TestRun,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Dyno {
        /// unique identifier or the name of this process on this dyno
        pub id: String,
        /// a URL to stream output from for debug runs or null for non-debug runs
        pub attach_url: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Pipeline {
        /// unique identifier or name of pipeline
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TestRun {
        /// unique identifier of a test run
        pub id: String,
    }
}

mod test_run {
    use chrono::offset::Utc;
    use chrono::DateTime;
    use serde_json::Value;

    /// Test Run
    ///
    /// Stability: prototype
    ///
    /// An execution or trial of one or more tests
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-run)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TestRun {
        /// the email of the actor triggering the test run
        pub actor_email: String,
        /// whether the test was run with an empty cache
        pub clear_cache: Option<bool>,
        /// the branch of the repository that the test run concerns
        pub commit_branch: String,
        /// the message for the commit under test
        pub commit_message: String,
        /// the SHA hash of the commit under test
        pub commit_sha: String,
        /// whether the test run was started for interactive debugging
        pub debug: bool,
        /// the app setup for the test run
        pub app_setup: Option<Value>, //TODO update this
        /// when test run was created
        pub created_at: DateTime<Utc>,
        /// the type of dynos used for this test-run
        pub dyno: Option<Dyno>,
        /// unique identifier of a test run
        pub id: String,
        /// human friendly message indicating reason for an error
        pub message: Option<String>,
        /// the auto incrementing test run number
        pub number: i64,
        /// the team that owns this test-run
        pub organization: Option<Organization>,
        /// pipeline
        pub pipeline: Pipeline,
        /// current state of the test run
        ///  one of:"pending" or "cancelled" or "creating" or "building" or "running" or "succeeded" or "failed" or "errored" or "debugging"
        pub status: String,
        /// The download location for the source code to be tested
        pub source_blob_url: String,
        /// when test-run was updated
        pub updated_at: DateTime<Utc>,
        /// user
        pub user: User,
        /// human friently warning emitted during the test run
        pub warning_message: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Dyno {
        /// dyno size (default: “standard-1X”)
        pub size: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Organization {
        /// unique name of team
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Pipeline {
        /// unique identifier or name of pipeline
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct User {
        /// whether to allow third party web activity tracking
        ///  default: true
        pub allow_tracking: Option<bool>,
        /// whether allowed to utilize beta Heroku features
        pub beta: Option<bool>,
        /// when account was created
        pub created_at: Option<DateTime<Utc>>,
        /// unique email address
        pub email: Option<String>,
        /// whether the user is federated and belongs to an Identity Provider
        pub federated: Option<bool>,
        /// identifier of an account
        pub id: String,
        /// Identity Provider details for federated users.
        pub identity_provider: Option<IdentityProvider>,
        /// when account last authorized with Heroku
        pub last_login: Option<DateTime<Utc>>,
        /// full name of the account owner
        pub name: Option<String>,
        /// SMS number of account
        pub sms_number: Option<String>,
        /// when account was suspended
        pub suspended_at: Option<String>,
        /// when account became delinquent
        pub delinquent_at: Option<DateTime<Utc>>,
        /// whether two-factor auth is enabled on the account
        pub two_factor_authentication: Option<bool>,
        /// when account was updated
        pub updated_at: Option<DateTime<Utc>>,
        /// whether account has been verified with billing information
        pub verified: Option<bool>,
        /// team selected by default
        pub default_organization: Option<DefaultOrganization>,
        /// team selected by default
        pub default_team: Option<DefaultTeam>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct IdentityProvider {
        /// unique identifier of this identity provider
        pub id: String,
        /// identity provider team
        pub team: Team,
        /// identity provider organization
        pub organization: Organization,
        /// identity provider owner
        pub owner: Owner,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Team {
        /// unique name of team
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Owner {
        /// unique identifier of the owner
        pub id: String,
        /// name of the owner
        pub name: String,
        /// type of the owner
        ///  one of:"team" or "enterprise-account"
        #[serde(rename = "type")]
        pub type_field: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct DefaultOrganization {
        /// unique identifier of team
        pub id: String,
        /// unique name of team
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct DefaultTeam {
        /// unique identifier of team
        pub id: String,
        /// unique name of team
        pub name: String,
    }
}
