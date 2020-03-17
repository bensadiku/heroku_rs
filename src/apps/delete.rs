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
    Domain
    DomainId
    DomainHostname
    Dyno
    DynoId
    DynoName
    LogDrain
    LogDrainId
    LogDrainUrl
    LogDrainToken
    Acm
    SNIEndpoints
    SNIEndpointId
    SNIEndpointName
    SSLEndpoints
    SSLEndpointId
    SSLEndpointName
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
        -> Domain = "domains"
        -> Dyno = "dynos"
        -> LogDrain = "log-drains"
        -> Acm = "acm"
        -> SNIEndpoints = "sni-endpoints"
        -> SSLEndpoints = "ssl-endpoints"
    @Webhooks
        => WebhookId
    @Collaborator
        => CollaboratorEmail
        => CollaboratorId
    @Domain
        => DomainHostname
        => DomainId
    @Dyno
        => DynoName
        => DynoId
    @LogDrain
        => LogDrainId
        => LogDrainUrl
        => LogDrainToken
    @SNIEndpoints
        => SNIEndpointId
        => SNIEndpointName
    @SSLEndpoints
        => SSLEndpointId
        => SSLEndpointName
      
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
        |=> app_domains -> Domain
        |=> app_dynos -> Dyno
        |=> app_log_drains -> LogDrain
        |=> app_acm -> Acm
        |=> app_sni_endpoints ->  SNIEndpoints
        |=> app_ssl_endpoints ->  SSLEndpoints
        |
    @Webhooks
        |
        |=> webhook_id -> WebhookId = id
    @Collaborator
        |
        |=> collaborator_email -> CollaboratorEmail = email
        |=> collaborator_id -> CollaboratorId = id
    @Domain
        |
        |=> domain_hostname -> DomainHostname = hostname
        |=> domain_id -> DomainId = id
    @Dyno
        |
        |=> dyno_name -> DynoName = name
        |=> dyno_id -> DynoId = id
    @LogDrain
        |
        |=> log_drain_id -> LogDrainId = id
        |=> log_drain_url -> LogDrainUrl = lurl
        |=> log_drain_token -> LogDrainToken = ltoken
    @SNIEndpoints
        |
        |=> sni_endpoint_id -> SNIEndpointId = id
        |=> sni_endpoint_name -> SNIEndpointName = name
    @SSLEndpoints
        |
        |=> ssl_endpoint_id -> SSLEndpointId = id
        |=> ssl_endpoint_name -> SSLEndpointName = name
);

exec!(App);
exec!(Apps);
exec!(Webhooks);
exec!(WebhookId);
exec!(BuildCache);
exec!(Collaborator);
exec!(CollaboratorEmail);
exec!(CollaboratorId);
exec!(Domain);
exec!(DomainId);
exec!(DomainHostname);
exec!(Dyno);
exec!(DynoId);
exec!(DynoName);
exec!(LogDrain);
exec!(LogDrainId);
exec!(LogDrainUrl);
exec!(LogDrainToken);
exec!(Acm);
exec!(SNIEndpoints);
exec!(SNIEndpointId);
exec!(SNIEndpointName);
exec!(SSLEndpoints);
exec!(SSLEndpointId);
exec!(SSLEndpointName);
