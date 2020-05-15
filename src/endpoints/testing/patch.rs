//Anything related to POST requests for Heroku tests goes here.
use super::TestRun;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Test Run Update
///
/// Update a test-run’s status.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-run-update)
pub struct TestRunUpdate<'a> {
    pub run_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: TestRunUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> TestRunUpdate<'a> {
    pub fn new(run_id: &'a str, message: Option<&'a str>, status: &'a str) -> TestRunUpdate<'a> {
        TestRunUpdate {
            run_id,
            params: TestRunUpdateParams { message, status },
        }
    }
}

/// Update a test run with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-run-update-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct TestRunUpdateParams<'a> {
    /// human friendly message indicating reason for an error. [Nullable]
    pub message: Option<&'a str>,
    /// current state of the test run
    ///  one of:"pending" or "cancelled" or "creating" or "building" or "running" or "succeeded" or "failed" or "errored" or "debugging" 
    pub status: &'a str,
}

impl<'a> HerokuEndpoint<TestRun, (), TestRunUpdateParams<'a>> for TestRunUpdate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("test-runs/{}", self.run_id)
    }
    fn body(&self) -> Option<TestRunUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
