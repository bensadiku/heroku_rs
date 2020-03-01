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
    @AppFeatures
        => AppFeaturesName  
        => AppFeaturesId  
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
