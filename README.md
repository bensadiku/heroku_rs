# heroku_rs

[![crates.io](https://img.shields.io/crates/v/heroku_rs.svg)](https://crates.io/crates/heroku_rs)
[![Documentation](https://docs.rs/heroku_rs/badge.svg)](https://docs.rs/heroku_rs)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/heroku_rs.svg)](./LICENSE)
[![CI](https://github.com/bensadiku/heroku_rs/workflows/Heroku/badge.svg)](https://github.com/bensadiku/heroku_rs/actions?query=workflow%3AHerokuCI)

## Intro

This crate provides some convenient Rust bindings for the [Heroku v3 API](https://devcenter.heroku.com/articles/platform-api-reference/).

See the [documentation](https://github.com/bensadiku/heroku_rs/blob/master/docs/ENDPOINTS.md) for more information on which endpoints are covered by the crate.

## Getting Started

Add the following to your `Cargo.toml` and run `cargo build`. Voila.
#### Note that version 0.6 and later uses tokio 1.0 

```toml
[dependencies]
heroku_rs = "0.6"
```

See [FEATURES](/docs/FEATURES.md) documentation for more information about the configurations of the crate.


###  - Example 

Here's a simple example which fetches the apps list.

```rust
use heroku_rs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create the client
    let heroku = HttpApiClient::create("API_KEY")?;
    //request all the apps
    let response = heroku.request(&AppList::new());
    
    //match response
    match response {
        Ok(apps) => println!("Success: {:#?}", apps),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
```
 For more documentation see the [docs](https://docs.rs/heroku_rs/).

 For more examples see the [directory](https://github.com/bensadiku/heroku_rs/tree/master/examples).


### Useful reads if you're not familiar with Heroku

[Heroku quickstart](https://devcenter.heroku.com/articles/platform-api-quickstart)

[Heroku API reference](https://devcenter.heroku.com/articles/platform-api-reference)

[Generating a heroku API key](https://help.heroku.com/PBGP6IDE/how-should-i-generate-an-api-key-that-allows-me-to-use-the-heroku-platform-api)

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

