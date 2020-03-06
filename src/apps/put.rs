//! Access the Apps portion of the Heroku API
imports!();
use crate::client::PutQueryBuilder;

new_type!(
    Apps
    App
    BuildPackInstallations
    LogDrains
    LogDrainId
    LogDrainUrl
    LogDrainToken
);

from!(
    @PutQueryBuilder
        -> Apps = "apps"
    @Apps
        => App
    @App
        -> BuildPackInstallations = "buildpack-installations"  
        -> LogDrains = "log-drains"    
    @LogDrains
        => LogDrainId
        => LogDrainUrl
        => LogDrainToken  
);

impl_macro!(
    @Apps
        |
        |=> app_name -> App = app_name_str
        |=> app_id -> App = app_id_str
    @App
        |=> app_buildpack_installation ->  BuildPackInstallations
        |=> app_log_drains ->  LogDrains
        |
    @LogDrains
        |
        |=> log_drain_id -> LogDrainId = id
        |=> log_drain_url -> LogDrainUrl = lurl
        |=> log_drain_token -> LogDrainToken = ltoken
);

exec!(Apps);
exec!(App);
exec!(BuildPackInstallations);
exec!(LogDrains);
exec!(LogDrainId);
exec!(LogDrainUrl);
exec!(LogDrainToken);