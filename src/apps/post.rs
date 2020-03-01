//!Create a new app.
imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Apps
    App
    Webhooks
    WebhookId
);

from!(
    @PostQueryBuilder
        -> Apps = "apps"
    @Apps
        => App
    @App
        -> Webhooks = "webhooks"    
    @Webhooks
        => WebhookId 
);

impl_macro!(
    @Apps
        |
        |=> app_name -> App = app_name_str
        |=> app_id -> App = app_id_str
    @App
        |=> app_webhooks ->  Webhooks
        |
    @Webhooks
        |
        |=> webhook_id -> WebhookId = id
);

exec!(Apps);
exec!(App);
exec!(Webhooks);
exec!(WebhookId);
