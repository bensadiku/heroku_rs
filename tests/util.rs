extern crate heroku_rs;

use heroku_rs::framework::{auth::Credentials, ApiEnvironment, HttpApiClient, HttpApiClientConfig};

pub const INVALID_ENDPOINT: &'static str =
    "The endpoint in the query was different from what was requested";

pub const TEST_ENDPOINT: &'static str = "https://api.notheroku.com/";

pub fn get_client() -> HttpApiClient {
    let credentials = Credentials::UserAuthToken {
        token: String::from("TOKEN_HERE"),
    };
    let api_client = HttpApiClient::new(
        credentials,
        HttpApiClientConfig::default(),
        ApiEnvironment::Custom(url::Url::parse(TEST_ENDPOINT).unwrap()),
    )
    .unwrap();
    api_client
}
