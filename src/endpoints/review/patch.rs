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
    pub fn new(
        pipeline_id: &'a str,
        automatic_review_apps: Option<bool>,
        destroy_stale_apps: Option<bool>,
        stale_days: Option<&'a str>,
        deploy_target_id: Option<&'a str>,
        deploy_target_type: Option<&'a str>,
        wait_for_ci: Option<bool>,
        base_name: Option<&'a str>,
    ) -> ReviewAppConfigUpdate<'a> {
        // tl;dr DeployTarget is nullable in the API and both fields should either provided or not.
        let deploy_type: Option<DeployTarget> = match (deploy_target_id, deploy_target_type) {
            (Some(target_id), Some(type_id)) => Some(DeployTarget {
                id: target_id,
                type_field: type_id
            }),
            (None, None) =>  None,
            _ => panic!("deploy_target_id and deploy_target_type have to be either both Some or both None, but not in-between!"),
        };

        ReviewAppConfigUpdate {
            pipeline_id,
            params: ReviewAppConfigUpdateParams {
                automatic_review_apps: automatic_review_apps,
                destroy_stale_apps: destroy_stale_apps,
                stale_days: stale_days,
                deploy_target: deploy_type,
                wait_for_ci: wait_for_ci,
                base_name: base_name,
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
