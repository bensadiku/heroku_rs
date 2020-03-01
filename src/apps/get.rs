//! Access the Apps portion of the Heroku API
imports!();
use crate::client::GetQueryBuilder;

// Declaration of types representing the various items under apps
new_type!(
    App
    Apps
    AppFeatures 
    AppFeaturesName 
    AppFeaturesId
    Webhooks
    WebhookId
    WebhookDelivery
    WebhookDeliveryId
    WebhookEvent
    WebhookEventId
    Build
    BuildId
    BuildpackInstallations
    Collaborator
    CollaboratorEmail
    CollaboratorId

);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> Apps = "apps"
    @Apps
        => App
    @App
        -> AppFeatures = "features"    
        -> Webhooks = "webhooks"    
        -> WebhookDelivery = "webhook-deliveries"    
        -> WebhookEvent = "webhook-events"    
        -> Build = "builds"    
        -> BuildpackInstallations = "buildpack-installations"    
        -> Collaborator = "collaborators"    
    @AppFeatures
        => AppFeaturesName  
        => AppFeaturesId  
    @Webhooks
        => WebhookId 
    @WebhookDelivery
        => WebhookDeliveryId 
    @WebhookEvent
        => WebhookEventId 
    @Build
        => BuildId 
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
        |=> app_features ->  AppFeatures
        |=> app_webhooks ->  Webhooks
        |=> app_webhook_deliveries ->  WebhookDelivery
        |=> app_webhook_events ->  WebhookEvent
        |=> app_builds ->  Build
        |=> app_buildpack_installations ->  BuildpackInstallations
        |=> app_collaborators ->  Collaborator
        |
    @AppFeatures
        |
        |=> feature_name -> AppFeaturesName = name
        |=> feature_id -> AppFeaturesId = id
    @Webhooks
        |
        |=> webhook_id -> WebhookId = id
    @WebhookDelivery
        |
        |=> webhook_delivery_id -> WebhookDeliveryId = id
    @WebhookEvent
        |
        |=> webhook_event_id -> WebhookEventId = id
    @Build
        |
        |=> build_id-> BuildId = id
    @Collaborator
        |
        |=> collaborator_id -> CollaboratorId = id
        |=> collaborator_email -> CollaboratorEmail = email
);

exec!(App);
exec!(Apps);
exec!(AppFeatures);
exec!(AppFeaturesName);
exec!(AppFeaturesId);
exec!(Webhooks);
exec!(WebhookId);
exec!(WebhookDelivery);
exec!(WebhookDeliveryId);
exec!(WebhookEvent);
exec!(WebhookEventId);
exec!(Build);
exec!(BuildId);
exec!(BuildpackInstallations);
exec!(Collaborator);
exec!(CollaboratorId);
exec!(CollaboratorEmail);
