use super::print_response;
use heroku_rs::prelude::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_name = "heroku-rs-tests";

    let token = dotenv::var("API_KEY").unwrap();

    // This will use the default headers & timeout for the HttpApiClientConfig
    // and will use the default production heroku endpoint
    let api_client = &HttpApiClient::create(&token)?;

    // create_dyno(api_client, app_name);
    create_slug(api_client, app_name);

    Ok(())
}

/// create a dyno
fn create_dyno<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let env_vars = heroku_env!["COLUMNS" => "80", "LINES" => "24",];
    let new_dyno_complex = &DynoCreate::new(app_id, "bash")
        .attach(true)
        .env(env_vars)
        .build();

    let response = api_client.request(new_dyno_complex);

    print_response(response);
}

/// create a slug
fn create_slug<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let process_types = heroku_env!["web" => "./bin/web -p $PORT"];

    let response = api_client.request(&SlugCreate::new(app_id, process_types));

    print_response(response);
}
