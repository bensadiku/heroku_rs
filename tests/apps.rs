use heroku_rs::endpoints::apps;
use util::assert_valid_url;
mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use heroku_rs::framework::apiclient::HerokuApiClient;
    // run with `cargo test -- --nocapture` for  the logs
    #[test]
    fn assert_valid_url_get_app_list() {
        let response = util::get_client().request(&apps::AppList {});
        let endpoint = format!("{}", "apps");
        assert_valid_url(response, endpoint)
    }

    #[test]
    fn assert_valid_url_get_app_details() {
        let app_id = "123xyz";
        let response = util::get_client().request(&apps::AppDetails { app_id: app_id });
        let endpoint = format!("{}{}", "apps/", app_id);
        assert_valid_url(response, endpoint)
    }
}
