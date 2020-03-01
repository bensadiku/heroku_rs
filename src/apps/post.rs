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
    Dyno
    DynoId
    DynoName
    DynoAction
    DynoActionStop
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
        -> Dyno = "dynos"     
    @Webhooks
        => WebhookId 
    @Dyno
        => DynoId 
        => DynoName 
    @DynoName
        -> DynoAction = "actions"
    @DynoId
        -> DynoAction = "actions"
    @DynoAction
        -> DynoActionStop = "stop"
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
        |=> app_dyno ->  Dyno
        |
    @Webhooks
        |
        |=> webhook_id -> WebhookId = id
    @Dyno
        |
        |=> dyno_id -> DynoId = id
        |=> dyno_name -> DynoName = name
    @DynoId
        |=> dyno_action ->  DynoAction
        |
    @DynoName
        |=> dyno_action ->  DynoAction
        |  
    @DynoAction
        |=> action_stop -> DynoActionStop
        |
);

exec!(Apps);
exec!(App);
exec!(Webhooks);
exec!(WebhookId);
exec!(Build);
exec!(Collaborator);
exec!(Domain);
exec!(Dyno);
exec!(DynoId);
exec!(DynoName);
exec!(DynoAction);
exec!(DynoActionStop);

