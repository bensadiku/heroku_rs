//!Create a new app.
imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Apps
    App
    Webhooks
    WebhookId
    Build
    Collaborator
    Domain
);

from!(
    @PostQueryBuilder
        -> Apps = "apps"
    @Apps
        => App
    @App
        -> Webhooks = "webhooks"    
        -> Build = "builds"    
        -> Collaborator = "collaborators"    
        -> Domain = "domains"    
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
        |=> app_build ->  Build
        |=> app_collaborator ->  Collaborator
        |=> app_domain ->  Domain
        |
    @Webhooks
        |
        |=> webhook_id -> WebhookId = id
);

exec!(Apps);
exec!(App);
exec!(Webhooks);
exec!(WebhookId);
exec!(Build);
exec!(Collaborator);
exec!(Domain);
