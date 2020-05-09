extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::pipelines;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    // create_pipeline(api_client);
    // get_pipeline(api_client);
    get_pipelines(api_client);
    // update_pipeline(api_client);
    // delete_pipeline(api_client); // Careful here :)

    // get_pipeline_latest_builds(api_client);

    // get_pipeline_pipeline_couplings(api_client);
    // get_user_pipeline_couplings(api_client);
    // get_team_pipeline_couplings(api_client);
    // get_pipeline_couplings(api_client);
    // create_pipeline_coupling(api_client);
    // get_pipeline_coupling(api_client);
    // update_pipeline_coupling(api_client);
    // get_app_pipeline_coupling(api_client);
    // delete_pipeline_coupling(api_client);

    // get_pipline_deployments(api_client);
    // create_pipeline_promotion(api_client);
    // get_pipeline_promotion(api_client);

    // get_pipeline_promotion_target_list(api_client);
    // get_pipeline_releases(api_client);
    // get_pipeline_stack(api_client);

    // create_pipeline_transfer(api_client);
}

// create pipeline transfer
fn create_pipeline_transfer<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let owner_type = "user";
    let new_owner_id = "OWNER_ID";
    let response = api_client.request(&pipelines::PipelineTransferCreate::new(
        pipeline_id,
        new_owner_id,
        owner_type,
    ));
    print_response(response);
}

// get pipline stack
fn get_pipeline_stack<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(&pipelines::PipelineStackDetails { pipeline_id });
    print_response(response);
}

// get pipline release
fn get_pipeline_releases<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(&pipelines::PipelineLatestReleaseList { pipeline_id });
    print_response(response);
}

// get pipline promotion target list
fn get_pipeline_promotion_target_list<T: HerokuApiClient>(api_client: &T) {
    let promotion_id = "PIPELINE_ID";
    let response = api_client.request(&pipelines::PipelinePromotionTargetList { promotion_id });
    print_response(response);
}

// get pipline promotion
fn get_pipeline_promotion<T: HerokuApiClient>(api_client: &T) {
    let promotion_id = "PIPELINE_ID";
    let response = api_client.request(&pipelines::PipelinePromotionDetails { promotion_id });
    print_response(response);
}

// create pipeline promotion
fn create_pipeline_promotion<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let source_app_id = "SOURCE_APP_ID";
    let target_app_id = "TARGET_APP_ID";
    let response = api_client.request(&pipelines::PipelinePromotionCreate::new(
        pipeline_id,
        source_app_id,
        target_app_id,
    ));
    print_response(response);
}

// get pipline deployments
fn get_pipline_deployments<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(&pipelines::PipelineDeploymentList { pipeline_id });
    print_response(response);
}

// delete pipeline coupling
fn delete_pipeline_coupling<T: HerokuApiClient>(api_client: &T) {
    let coupling_id = "COUPLING_ID";
    let response = api_client.request(&pipelines::PipelineCouplingDelete { coupling_id });
    print_response(response);
}

// get app pipeline coupling details
fn get_app_pipeline_coupling<T: HerokuApiClient>(api_client: &T) {
    let app_id = "APP_ID";
    let response = api_client.request(&pipelines::PipelineCouplingByAppDetails { app_id });
    print_response(response);
}

// update pipeline coupling
fn update_pipeline_coupling<T: HerokuApiClient>(api_client: &T) {
    let coupling_id = "COUPLING_ID";
    let response = api_client.request(
        &pipelines::PipelineCouplingUpdate::new(coupling_id)
            .stage("development")
            .build(),
    );
    print_response(response);
}

// get pipeline coupling details
fn get_pipeline_coupling<T: HerokuApiClient>(api_client: &T) {
    let coupling_id = "COUPLING_ID";
    let response = api_client.request(&pipelines::PipelineCouplingDetails { coupling_id });
    print_response(response);
}

// craete pipeline coupling
fn create_pipeline_coupling<T: HerokuApiClient>(api_client: &T) {
    let app = "APP_ID"; // can be app name or app id
    let pipeline = "PIPELINE_ID"; // pipeline id
    let stage = "test";
    let response = api_client.request(&pipelines::PipelineCouplingCreate::new(
        app, pipeline, stage,
    ));
    print_response(response);
}

// get pipeline couplings
fn get_pipeline_couplings<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&pipelines::PipelineCouplingList {});
    print_response(response);
}

// get team pipeline couplings
fn get_team_pipeline_couplings<T: HerokuApiClient>(api_client: &T) {
    let team_id = "TEAM_ID";
    let response = api_client.request(&pipelines::PipelineCouplingByTeamList { team_id });
    print_response(response);
}

// get user pipeline couplings
fn get_user_pipeline_couplings<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&pipelines::PipelineCouplingByUserList {});
    print_response(response);
}

// get pipeline couplings by pipeline id
fn get_pipeline_pipeline_couplings<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(&pipelines::PipelineCouplingByPipelineList { pipeline_id });
    print_response(response);
}

// get pipeline latest builds
fn get_pipeline_latest_builds<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(&pipelines::PipelineLatestBuildsList { pipeline_id });
    print_response(response);
}

// delete pipeline
fn delete_pipeline<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(&pipelines::PipelineDelete { pipeline_id });
    print_response(response);
}

// update pipeline
fn update_pipeline<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(
        &pipelines::PipelineUpdate::new(pipeline_id)
            .name("my-renamed-pipe")
            .build(),
    );
    print_response(response);
}

// get pipeline info
fn get_pipeline<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(&pipelines::PipelineDetails { pipeline_id });
    print_response(response);
}

// get pipeline list
fn get_pipelines<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&pipelines::PipelineList {});
    print_response(response);
}

// create a new pipeline
fn create_pipeline<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&pipelines::PipelineCreate::new("my-pipe"));
    print_response(response);
}
