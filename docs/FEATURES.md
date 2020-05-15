## Features

Heroku_rs features can be toggled by the `Cargo.toml` file.


The default configuration contains everything from the crate. All the available Heroku endpoints, and the builder pattern for each of the endpoints.

 ### Setting up the default configuration.

Add this to your `Cargo.toml` file and run `cargo build`. That will download the crate and make it ready for use.

#### Example:

```toml
[dependencies]
heroku_rs = "0.5"
```

### Setting up a custom configuration.

If you do not need every endpoints from the API, you can configure only the ones you want.


Available configurable endpoints are: `account`, `addons`, `apps`, `builds`, `collaborators`, `config_vars`, `custom`, `domains`, `dynos`, `formations`, `logs`, `misc`, `oauth`, `pipelines`, `releases`, `review`, `slugs`, `space`, `teams`, `testing`.

Other features are: `builder`. Note: When builder feature is activated, if will be enabled for every endpoints that you added to the config.

#### Example:

```toml
[dependencies.heroku_rs]
default-features = false
features = ["apps","account","builder"]
version = "0.5"
```
