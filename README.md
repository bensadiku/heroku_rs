# heroku_rs

[![crates.io](https://img.shields.io/crates/v/heroku_rs.svg)](https://crates.io/crates/heroku_rs)
[![Documentation](https://docs.rs/heroku_rs/badge.svg)](https://docs.rs/heroku_rs)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/heroku_rs.svg)](./LICENSE)
[![CI](https://github.com/bensadiku/heroku_rs/workflows/Heroku/badge.svg)](https://github.com/bensadiku/heroku_rs/actions?query=workflow%3AHerokuCI)

## Intro

This crate is a API wrapper for the [Heroku v3 API](https://devcenter.heroku.com/articles/platform-api-reference/).

See the [documentation](https://github.com/bensadiku/heroku_rs/blob/master/docs/ENDPOINTS.md) for more information on which endpoints are covered by the crate. 

## Getting Started
Add the following to your `Cargo.toml` and run `cargo build`. Voila.

```toml
[dependencies]
heroku_rs = "0.4.1"
```

Here's a simple example which fetches the apps list. At the moment, the client is blocking by default. For more examples see the [examples directory](https://github.com/bensadiku/heroku_rs/tree/master/examples).

```rust
use heroku_rs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = HttpApiClient::create("API_KEY")?;
    let response = api_client.request(&apps::AppList {});

    match response {
        Ok(success) => println!("Success: {:#?}", success),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
```

You can also call custom endpoints that have not been supported by the library yet. e.g. :


```rust
use heroku_rs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = HttpApiClient::create("API_KEY")?;

    // This will do a GET request on https://api.heroku.com/apps/my_app_name_here
    let query = format!("{}{}", "apps/", "my_app_name_here");
    let method = Method::Get;
    let response = api_client.request(&custom::CustomEndpointSimple::new(query, method);

    Ok(())
}    

```

See more [examples](https://github.com/bensadiku/heroku_rs/blob/master/examples/src/custom_examples.rs) on how to use these custom endpoints.

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


