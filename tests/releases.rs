use heroku_rs::endpoints::releases;
use util::assert_valid_url;
mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use heroku_rs::framework::apiclient::HerokuApiClient;
    // run with `cargo test -- --nocapture` for  the logs

    #[test]
    fn assert_valid_url_get_release_list() {
        let app_id = "123xyz";
        let response = util::get_client().request(&releases::ReleaseList {
            app_id: app_id,
        });

        let endpoint = format!("{}{}{}", "apps/", app_id, "/releases");
        assert_valid_url(response, endpoint)
    }

    #[test]
    fn assert_valid_url_get_app_info() {
        let app_id = "123xyz";
        let release_id = "456abc";
        let response = util::get_client().request(&releases::ReleaseInfo {
            app_id: app_id,
            release_id: release_id,
        });

        let endpoint = format!("{}{}{}{}", "apps/", app_id, "/releases/", release_id);
        assert_valid_url(response, endpoint)
    }

    #[test]
    fn assert_valid_url_release_create() {
        let app_id = "123xyz";
        let response = util::get_client().request(&releases::ReleaseCreate {
            app_id: app_id,
            params: releases::ReleaseCreateParams {
                slug: "fooslug",
                description: Some("releasing the thing"),
            },
        });
        let endpoint = format!("{}{}{}", "apps/", app_id, "/releases");
        assert_valid_url(response, endpoint)
    }

    #[test]
    fn assert_valid_url_release_rollback() {
        let app_id = "123xyz";

        let response = util::get_client().request(&releases::ReleaseRollback {
            app_id: app_id,
            params: releases::ReleaseRollbackParams {
                release: "v40",
            },
        });
        let endpoint = format!("{}{}{}", "apps/", app_id, "/releases");
        assert_valid_url(response, endpoint)
    }
}
