use heroku_rs::endpoints::apps;

mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use heroku_rs::framework::apiclient::HerokuApiClient;
    // run with `cargo test -- --nocapture` for  the logs
    #[test]
    fn assert_valid_url() {
        let response = util::get_client().request(&apps::AppList {});

        match response {
            Ok(success) => {
                // This should never succeed because the URL is not valid!
                println!("Success: {:#?}", success);
                panic!("Got a successful response on a inexistent api call");
            }
            Err(e) => {
                let api = format!("{}{}", util::TEST_ENDPOINT, "apps");
                let pass = e.to_string().contains(&api);
                if !pass {
                    panic!("{}", util::INVALID_ENDPOINT)
                }
                println!("Error: {}", e);
            }
        }
    }
}
