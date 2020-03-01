//!Update an existing app.
imports!();
use crate::client::PatchQueryBuilder;

new_type!(
    Apps
    App
    AppFeatures 
    AppFeaturesName 
    AppFeaturesId
    Webhooks
    WebhookId
);

from!(
    @PatchQueryBuilder
        -> Apps = "apps"
    @Apps
        => App
    @App
        -> AppFeatures = "features"
        -> Webhooks = "webhooks"    
    @AppFeatures
        => AppFeaturesName  
        => AppFeaturesId 
    @Webhooks
        => WebhookId
);

impl_macro!(
    @Apps
        |
        |=> app_name -> App = app_name_str
        |=> app_id -> App = app_id_str
    @App
        |=> app_features ->  AppFeatures
        |=> app_webhooks ->  Webhooks
        |
    @AppFeatures
        |
        |=> feature_name -> AppFeaturesName = name
        |=> feature_id -> AppFeaturesId = id
    @Webhooks
        |
        |=> webhook_id -> WebhookId = id
);

exec!(App);
exec!(Apps);
exec!(AppFeatures);
exec!(AppFeaturesName);
exec!(AppFeaturesId);
exec!(Webhooks);
exec!(WebhookId);