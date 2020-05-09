use heroku_rs::endpoints::pipelines;
use util::assert_valid_url;
mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use heroku_rs::framework::apiclient::HerokuApiClient;

    #[test]
    fn assert_valid_url_get_pipeline_list() {
        let response = util::get_client().request(&pipelines::PipelineList {});
        let endpoint = format!("{}", "pipelines");
        assert_valid_url(response, endpoint)
    }

    #[test]
    fn assert_valid_url_get_pipeline_details() {
        let pipeline_id = "123xyz";
        let response = util::get_client().request(&pipelines::PipelineDetails {
            pipeline_id: pipeline_id,
        });
        let endpoint = format!("{}{}", "pipelines/", pipeline_id);
        assert_valid_url(response, endpoint)
    }

    #[test]
    fn assert_valid_url_get_pipeline_coupling_details() {
        let coupling_id = "123xyz";
        let response = util::get_client().request(&pipelines::PipelineCouplingDetails {
            coupling_id: coupling_id,
        });
        let endpoint = format!("{}{}", "pipeline-couplings/", coupling_id);
        assert_valid_url(response, endpoint)
    }

    #[test]
    fn assert_valid_url_get_pipeline_coupling_list() {
        let response = util::get_client().request(&pipelines::PipelineCouplingList {});
        let endpoint = format!("{}", "pipeline-couplings");
        assert_valid_url(response, endpoint)
    }
}
