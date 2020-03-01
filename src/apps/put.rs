//! Access the Apps portion of the Heroku API
imports!();
use crate::client::PutQueryBuilder;

new_type!(
    Apps
    App
    BuildPackInstallations
);

from!(
    @PutQueryBuilder
        -> Apps = "apps"
    @Apps
        => App
    @App
        -> BuildPackInstallations = "buildpack-installations"    
);

impl_macro!(
    @Apps
        |
        |=> app_name -> App = app_name_str
        |=> app_id -> App = app_id_str
    @App
        |=> app_buildpack_installation ->  BuildPackInstallations
        |
);

exec!(Apps);
exec!(App);
exec!(BuildPackInstallations);
