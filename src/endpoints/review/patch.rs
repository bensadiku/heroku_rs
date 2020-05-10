//Anything related to PATCH requests for review app and it's properties goes here.
use super::ReviewAppConfig;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Review App Configuration Update
///
/// Update review app configuration for a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-configuration-update)
pub struct ReviewAppConfigUpdate<'a> {
    pub pipeline_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: ReviewAppConfigUpdateParams<'a>,
}

impl<'a> ReviewAppConfigUpdate<'a> {
    pub fn new(pipeline_id: &'a str) -> ReviewAppConfigUpdate<'a> {
        ReviewAppConfigUpdate {
            pipeline_id,
            params: ReviewAppConfigUpdateParams {
                automatic_review_apps: None,
                destroy_stale_apps: None,
                stale_days: None,
                deploy_target: None,
                wait_for_ci: None,
                base_name: None,
            },
        }
    }

    pub fn base_name(&mut self, base_name: &'a str) -> &mut Self {
        self.params.base_name = Some(base_name);
        self
    }

    pub fn wait_for_ci(&mut self, wait_for_ci: bool) -> &mut Self {
        self.params.wait_for_ci = Some(wait_for_ci);
        self
    }

    pub fn deploy_target(&mut self, id: &'a str, t_type: &'a str) -> &mut Self {
        self.params.deploy_target = Some(DeployTarget {
            id: id,
            type_field: t_type,
        });
        self
    }

    pub fn stale_days(&mut self, stale_days: &'a str) -> &mut Self {
        self.params.stale_days = Some(stale_days);
        self
    }

    pub fn destroy_stale_apps(&mut self, destroy_stale_apps: bool) -> &mut Self {
        self.params.destroy_stale_apps = Some(destroy_stale_apps);
        self
    }

    pub fn automatic_review_apps(&mut self, automatic_review_apps: bool) -> &mut Self {
        self.params.automatic_review_apps = Some(automatic_review_apps);
        self
    }

    pub fn build(&self) -> ReviewAppConfigUpdate<'a> {
        ReviewAppConfigUpdate {
            pipeline_id: self.pipeline_id,
            params: ReviewAppConfigUpdateParams {
                automatic_review_apps: self.params.automatic_review_apps,
                destroy_stale_apps: self.params.destroy_stale_apps,
                stale_days: self.params.stale_days,
                deploy_target: self.params.deploy_target.clone(),
                wait_for_ci: self.params.wait_for_ci,
                base_name: self.params.base_name,
            },
        }
    }
}

/// Update review app configurations with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-configuration-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct ReviewAppConfigUpdateParams<'a> {
    /// If true, this will trigger the creation of review apps when pull-requests are opened in the repo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_review_apps: Option<bool>,
    /// If true, this will trigger automatic deletion of review apps when they’re stale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_stale_apps: Option<bool>,
    /// If destroy_stale_apps is true, then apps will be destroyed after this many days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_days: Option<&'a str>,
    /// Provides a key/value pair to specify whether to use a common runtime or a private space. [Nullable]
    pub deploy_target: Option<DeployTarget<'a>>,
    /// If true, review apps will only be created when CI passes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_ci: Option<bool>,
    /// A unique prefix that will be used to create review app names. [Nullable]
    pub base_name: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DeployTarget<'a> {
    /// unique identifier of deploy target
    ///  pattern: `(^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$
    pub id: &'a str,
    #[serde(rename = "type")]
    /// type of deploy target
    ///  pattern: `(^space$
    pub type_field: &'a str,
}

impl<'a> HerokuEndpoint<ReviewAppConfig, (), ReviewAppConfigUpdateParams<'a>>
    for ReviewAppConfigUpdate<'a>
{
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("pipelines/{}/review-app-config", self.pipeline_id)
    }
    fn body(&self) -> Option<ReviewAppConfigUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
