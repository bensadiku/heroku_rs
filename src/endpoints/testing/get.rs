//Anything related to GET requests for Heroku tests goes here.
use super::{TestCase, TestNode, TestRun};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Test Case List
///
/// List test cases
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-case-list)
pub struct TestCaseList<'a> {
    /// run_id is the test run unique identifier
    pub run_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TestCaseList<'a> {
    pub fn new(run_id: &'a str) -> TestCaseList<'a> {
        TestCaseList { run_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TestCase>> for TestCaseList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("test-runs/{}/test-cases", self.run_id)
    }
}

/// Test Node List
///
/// List test nodes
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-node-list)
pub struct TestNodeList<'a> {
    /// run_id is the test run unique identifier
    pub run_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TestNodeList<'a> {
    pub fn new(run_id: &'a str) -> TestNodeList<'a> {
        TestNodeList { run_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TestNode>> for TestNodeList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("test-runs/{}/test-nodes", self.run_id)
    }
}

/// Test Run Info
///
/// Info for existing test-run.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-run-info)
pub struct TestRunDetails<'a> {
    /// run_id is the test run unique identifier
    pub run_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TestRunDetails<'a> {
    pub fn new(run_id: &'a str) -> TestRunDetails<'a> {
        TestRunDetails { run_id }
    }
}

impl<'a> HerokuEndpoint<TestRun> for TestRunDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("test-runs/{}", self.run_id)
    }
}

/// Test Run List
///
/// List existing test-runs for a pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-run-list)
pub struct TestRunList<'a> {
    /// pipeline_id is the test run pipeline identifier
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TestRunList<'a> {
    pub fn new(pipeline_id: &'a str) -> TestRunList<'a> {
        TestRunList { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TestRun>> for TestRunList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/test-runs", self.pipeline_id)
    }
}

/// Test Run Info By Pipeline
///
/// Info for existing test-run by Pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-run-info-by-pipeline)
pub struct TestRunDetailsByPipeline<'a> {
    /// pipeline_id is the test run pipeline identifier
    pub pipeline_id: &'a str,
    /// run_id is the test run unique identifier
    pub run_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TestRunDetailsByPipeline<'a> {
    pub fn new(pipeline_id: &'a str, run_id: &'a str) -> TestRunDetailsByPipeline<'a> {
        TestRunDetailsByPipeline {
            pipeline_id,
            run_id,
        }
    }
}

impl<'a> HerokuEndpoint<TestRun> for TestRunDetailsByPipeline<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/test-runs/{}", self.pipeline_id, self.run_id)
    }
}
