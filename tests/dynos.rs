use heroku_rs::endpoints::dynos;
use util::assert_valid_url;
mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use heroku_rs::framework::apiclient::HerokuApiClient;
    // run with `cargo test -- --nocapture` for  the logs

    #[test]
    fn assert_valid_url_get_dyno_list() {
        let app_id = "123xyz";
        let response = util::get_client().request(&dynos::DynoList {
            app_id: app_id.to_owned(),
        });
        let endpoint = format!("{}{}{}", "apps/", app_id, "/dynos");
        assert_valid_url(response, endpoint)
    }

    #[test]
    fn assert_valid_url_dyno_stop() {
        let app_id = "123xyz";
        let dyno_id = "xyz123";
        let response = util::get_client().request(&dynos::DynoActionStop {
            app_id: app_id.to_owned(),
            dyno_id: dyno_id.to_owned(),
        });
        let endpoint = format!(
            "{}{}{}{}{}",
            "apps/", app_id, "/dynos/", dyno_id, "/actions/stop"
        );
        assert_valid_url(response, endpoint)
    }

    #[test]
    fn assert_valid_url_dyno_create() {
        let app_id = "123xyz";
        let response = util::get_client().request(&dynos::DynoCreate {
            app_id: app_id.to_owned(),
            params: dynos::DynoCreateParams {
                command: "bash".to_owned(),
                attach: None,
                env: None,
                force_no_tty: None,
                size: None,
                time_to_live: None,
                r#type: None,
            }
        });
        let endpoint = format!(
            "{}{}{}",
            "apps/", app_id, "/dynos"
        );
        assert_valid_url(response, endpoint)
    }
}
