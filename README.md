# heroku-rs

[![crates.io](https://img.shields.io/crates/v/heroku_rs.svg)](https://crates.io/crates/heroku_rs)
[![Documentation](https://docs.rs/heroku_rs/badge.svg)](https://docs.rs/heroku_rs)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/heroku_rs.svg)](./LICENSE)
[![CI](https://github.com/bensadiku/heroku_rs/workflows/Heroku/badge.svg)](https://github.com/bensadiku/heroku_rs/actions?query=workflow%3AHerokuCI)

**This project is currently under development**

## Intro

This is a rust api wrapper for the [Heroku v3 API](https://devcenter.heroku.com/articles/platform-api-reference/).

## Getting Started
Add the following to your `Cargo.toml`

```toml
[dependencies]
heroku_rs = "0.3"
```
run: `cargo build`

Here's a simple example which fetches the apps list. At the moment, the client is blocking by default.

```rust
use heroku_rs::framework::{
    auth::Credentials,
    response::{ApiResponse, ApiResult},
    apiclient::HerokuApiClient,
    ApiEnvironment, HttpApiClient, HttpApiClientConfig,
};
use heroku_rs::endpoints::apps;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let credentials = Credentials::UserAuthToken {
        token: String::from("TOKEN_HERE"),
    };

    let api_client = HttpApiClient::new(
        credentials,
        HttpApiClientConfig::default(),
        ApiEnvironment::Production,
    )?;

    let response = api_client.request(&apps::AppList {});

    match response {
        Ok(success) => println!("Success: {:#?}", success),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
```
    
### Useful links

[Heroku quickstart](https://devcenter.heroku.com/articles/platform-api-quickstart) 

[Heroku API reference](https://devcenter.heroku.com/articles/platform-api-reference)

[Generating a heroku API key](https://help.heroku.com/PBGP6IDE/how-should-i-generate-an-api-key-that-allows-me-to-use-the-heroku-platform-api)



## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


