//!Update an existing app.
imports!();
use crate::client::PatchQueryBuilder;

new_type!(
    Apps
    App
    AppFeatures 
    AppFeaturesName 
    AppFeaturesId
);

from!(
    @PatchQueryBuilder
        -> Apps = "apps"
    @Apps
        => App
    @App
        -> AppFeatures = "features"
    @AppFeatures
        => AppFeaturesName  
        => AppFeaturesId   
);

impl_macro!(
    @Apps
        |
        |=> app_name -> App = app_name_str
        |=> app_id -> App = app_id_str
    @App
        |=> app_features ->  AppFeatures
        |
    @AppFeatures
        |
        |=> feature_name -> AppFeaturesName = name
        |=> feature_id -> AppFeaturesId = id
);

exec!(App);
exec!(Apps);
exec!(AppFeatures);
exec!(AppFeaturesName);
exec!(AppFeaturesId);
