//Anything related to POST requests for Heroku tests goes here.
use super::TestRun;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Test Run Create
///
/// Create a new test-run.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-run-create)
pub struct TestRunCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: TestRunCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> TestRunCreate<'a> {
    pub fn new(
        commit_branch: &'a str,
        commit_message: &'a str,
        commit_sha: &'a str,
        pipeline: &'a str,
        source_blob_url: &'a str,
    ) -> TestRunCreate<'a> {
        TestRunCreate {
            params: TestRunCreateParams {
                commit_branch: commit_branch,
                commit_message: commit_message,
                commit_sha: commit_sha,
                debug: None,
                organization: None,
                pipeline: pipeline,
                source_blob_url: source_blob_url,
            },
        }
    }

    pub fn debug(&mut self, debug: bool) -> &mut Self {
        self.params.debug = Some(debug);
        self
    }

    pub fn organization(&mut self, organization: &'a str) -> &mut Self {
        self.params.organization = Some(organization);
        self
    }

    pub fn build(&self) -> TestRunCreate<'a> {
        TestRunCreate {
            params: TestRunCreateParams {
                commit_branch: self.params.commit_branch,
                commit_message: self.params.commit_message,
                commit_sha: self.params.commit_sha,
                debug: self.params.debug,
                organization: self.params.organization,
                pipeline: self.params.pipeline,
                source_blob_url: self.params.source_blob_url,
            },
        }
    }
}

/// Create a new test run with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-run-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct TestRunCreateParams<'a> {
    /// the branch of the repository that the test run concerns
    pub commit_branch: &'a str,
    /// the message for the commit under test
    pub commit_message: &'a str,
    /// the SHA hash of the commit under test
    pub commit_sha: &'a str,
    /// whether the test run was started for interactive debugging
    pub debug: Option<bool>,
    /// unique name or identifier of team
    pub organization: Option<&'a str>,
    /// unique identifier or name of pipeline
    pub pipeline: &'a str,
    /// The download location for the source code to be tested
    pub source_blob_url: &'a str,
}

impl<'a> HerokuEndpoint<TestRun, (), TestRunCreateParams<'a>> for TestRunCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("test-runs")
    }
    fn body(&self) -> Option<TestRunCreateParams<'a>> {
        Some(self.params.clone())
    }
}
