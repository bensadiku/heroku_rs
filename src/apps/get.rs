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
    ConfigVars
    Releases
    ReleaseId
    ReleaseVersion
    Domain
    DomainId
    DomainHostname
    Dyno
    DynoId
    DynoName

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
        -> ConfigVars = "config-vars"    
        -> Releases = "releases"    
        -> Domain = "domains"    
        -> Dyno = "dynos"    
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
    @Releases
        => ReleaseVersion
        => ReleaseId
    @ReleaseVersion
        -> ConfigVars = "config-vars"
    @ReleaseId
        -> ConfigVars = "config-vars"
    @Domain
        => DomainHostname
        => DomainId
    @Dyno
        => DynoName
        => DynoId
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
        |=> app_config_vars ->  ConfigVars
        |=> app_releases->  Releases
        |=> app_domains ->  Domain
        |=> app_dynos ->  Dyno
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
    @Releases
        |
        |=> release_id -> ReleaseId = id
        |=> release_version -> ReleaseVersion = version
    @ReleaseId
        |=> release_config_vars ->  ConfigVars
        |
    @ReleaseVersion
        |=> release_config_vars ->  ConfigVars 
        |    
    @Domain
        |
        |=> domain_id -> DomainId = id
        |=> domain_hostname -> DomainHostname = hostname
    @Dyno
        |
        |=> dyno_id -> DynoId = id
        |=> dyno_name -> DynoName = name
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
exec!(ConfigVars);
exec!(Releases);
exec!(ReleaseId);
exec!(ReleaseVersion);
exec!(Domain);
exec!(DomainId);
exec!(DomainHostname);
exec!(Dyno);
exec!(DynoId);
exec!(DynoName);


