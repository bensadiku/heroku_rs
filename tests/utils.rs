use heroku_rs::client::Heroku;
use std::env;

const INVALID_API_KEY: &'static str =
    "Your auth_token file is not setup properly. \
     Refer to the contributing guidelines to see how to set one up.";

pub const FAILED_HEROKU_CONNECTION: &'static str =
    "Unable to connect with HEROKU. \
     Make sure you have configured your access token correctly.";

pub const TEST_APP_NAME: &'static str = "heroku-rs-tests";


// This key is passed by github actions
pub fn setup_heroku_connection() -> Heroku {
    Heroku::new(&env::var("AUTH_KEY").expect(INVALID_API_KEY)).unwrap()
}

