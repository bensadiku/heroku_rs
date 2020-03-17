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
    ConfigVars
    Formations
    FormationId
    FormationType
    SNIEndpoints
    SNIEndpointName
    SNIEndpointId
    SSLEndpoints
    SSLEndpointId
    SSLEndpointName
    Addons
    AddonId
    AddonName
    Acm
);

from!(
    @PatchQueryBuilder
        -> Apps = "apps"
    @Apps
        => App
    @App
        -> AppFeatures = "features"
        -> Webhooks = "webhooks"    
        -> ConfigVars = "config-vars"    
        -> Formations = "formation"    
        -> SNIEndpoints = "sni-endpoints"    
        -> SSLEndpoints = "ssl-endpoints"    
        -> Addons = "addons"    
        -> Acm = "acm"    
    @AppFeatures
        => AppFeaturesName  
        => AppFeaturesId 
    @Webhooks
        => WebhookId
    @Formations
        => FormationId
        => FormationType
    @SNIEndpoints
        => SNIEndpointId
        => SNIEndpointName
    @SSLEndpoints
        => SSLEndpointId
        => SSLEndpointName
    @Addons
        => AddonId
        => AddonName
);

impl_macro!(
    @Apps
        |
        |=> app_name -> App = app_name_str
        |=> app_id -> App = app_id_str
    @App
        |=> app_features ->  AppFeatures
        |=> app_webhooks ->  Webhooks
        |=> app_config_vars ->  ConfigVars
        |=> app_formations ->  Formations
        |=> app_sni_endpoint ->  SNIEndpoints
        |=> app_ssl_endpoint ->  SSLEndpoints
        |=> app_addons ->  Addons
        |=> app_acm ->  Acm
        |
    @AppFeatures
        |
        |=> feature_name -> AppFeaturesName = name
        |=> feature_id -> AppFeaturesId = id
    @Webhooks
        |
        |=> webhook_id -> WebhookId = id
    @Formations
        |
        |=> formation_id -> FormationId = id
        |=> formation_type -> FormationType = ftype
    @SNIEndpoints
        |
        |=> sni_endpoint_id -> SNIEndpointId = id
        |=> sni_endpoint_name -> SNIEndpointName = name
    @SSLEndpoints
        |
        |=> ssl_endpoint_id -> SSLEndpointId = id
        |=> ssl_endpoint_name -> SSLEndpointName = name
    @Addons
        |
        |=> addon_id -> AddonId = id
        |=> addon_name -> AddonName = name
);

exec!(App);
exec!(Apps);
exec!(AppFeatures);
exec!(AppFeaturesName);
exec!(AppFeaturesId);
exec!(Webhooks);
exec!(WebhookId);
exec!(ConfigVars);
exec!(Formations);
exec!(FormationId);
exec!(FormationType);
exec!(SNIEndpoints);
exec!(SNIEndpointId);
exec!(SNIEndpointName);
exec!(SSLEndpoints);
exec!(SSLEndpointId);
exec!(SSLEndpointName);
exec!(Addons);
exec!(AddonId);
exec!(AddonName);
exec!(Acm);
