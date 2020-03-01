//! Access the Apps portion of the Heroku API
imports!();
use crate::client::DeleteQueryBuilder;

// Declaration of types representing the various items under apps
new_type!(
    App
    Apps
    Webhooks
    WebhookId
    BuildCache

);

// From implementations for conversion
from!(
    @DeleteQueryBuilder
        -> Apps = "apps"
    @Apps
        => App
    @App
        -> Webhooks = "webhooks"
        -> BuildCache = "build-cache"
    @Webhooks
        => WebhookId
      
);

// impls of each type
impl_macro!(
    @Apps
        |
        |=> app_name -> App = app_name_str
        |=> app_id -> App = app_id_str
    @App
        |=> app_webhooks -> Webhooks
        |=> app_build_cache -> BuildCache
        |
    @Webhooks
        |
        |=> webhook_id -> WebhookId = id
);

exec!(App);
exec!(Apps);
exec!(Webhooks);
exec!(WebhookId);
exec!(BuildCache);
