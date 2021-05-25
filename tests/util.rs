extern crate heroku_rs;

use heroku_rs::framework::{
    auth::Credentials,
    response::{ApiResponse, ApiResult},
    ApiEnvironment, HttpApiClient, HttpApiClientConfig,
};
use std::time::Duration;

pub const INVALID_ENDPOINT: &'static str =
    "The endpoint in the query was different from what was requested";

pub const TEST_ENDPOINT: &'static str = "https://api.notheroku.com/";

pub fn get_client() -> HttpApiClient {
    let credentials = Credentials::UserAuthToken {
        token: String::from("TOKEN_HERE"),
    };
    let api_client = HttpApiClient::new(
        credentials,
        HttpApiClientConfig {
            http_timeout: Duration::from_secs(10),
            default_headers: http::HeaderMap::default(),
        },
        ApiEnvironment::Custom(url::Url::parse(TEST_ENDPOINT).unwrap()),
    )
    .unwrap();
    api_client
}

#[cfg(test)]
mod tests {
    use super::*;
    use heroku_rs::endpoints::misc;
    use heroku_rs::framework::apiclient::HerokuApiClient;
    // run with `cargo test -- --nocapture` for  the logs

    #[test]
    fn assert_valid_url_get_dyno_list() {
        let region_id = "123xyz";
        let response = get_client().request(&misc::RegionDetails {
            region_id: region_id,
        });
        let endpoint = format!("{}{}", "regions/", region_id);
        assert_valid_url(response, endpoint)
    }
}

pub fn assert_valid_url<T: ApiResult>(response: ApiResponse<T>, second_part: String) {
    match response {
        Ok(success) => {
            // This should never succeed because the URL is not valid!
            println!("Success: {:#?}", success);
            panic!("{}", "Got a successful response on a inexistent api call");
        }
        Err(e) => {
            let mut error = e.to_string();
            let api = format!("{}{}", TEST_ENDPOINT, second_part);
            if !error.contains(&api) {
                // Remove the range up until the ( from the string
                let beta_offset = error.find('(').unwrap_or(error.len());
                let mut error: String = error.drain(beta_offset..).collect();

                // Remove the range up from the ) from the string
                let alpha_offset = error.find(')').unwrap_or(error.len());
                let error: String = error.drain(..alpha_offset).collect();

                panic!("{}. Excpected: {}. Got: {}", INVALID_ENDPOINT, api, error);
            }
            println!("Error: {}", e);
        }
    }
}
