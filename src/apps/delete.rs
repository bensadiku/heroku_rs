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
    Collaborator
    CollaboratorEmail
    CollaboratorId

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
        -> Collaborator = "collaborators"
    @Webhooks
        => WebhookId
    @Collaborator
        => CollaboratorEmail
        => CollaboratorId
      
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
        |=> app_collaborators -> Collaborator
        |
    @Webhooks
        |
        |=> webhook_id -> WebhookId = id
    @Collaborator
        |
        |=> collaborator_email -> CollaboratorEmail = email
        |=> collaborator_id -> CollaboratorId = id
);

exec!(App);
exec!(Apps);
exec!(Webhooks);
exec!(WebhookId);
exec!(BuildCache);
exec!(Collaborator);
exec!(CollaboratorEmail);
exec!(CollaboratorId);
