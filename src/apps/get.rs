//! Access the Apps portion of the Heroku API
imports!();
use crate::client::GetQueryBuilder;

// Declaration of types representing the various items under apps
new_type!(
   Apps
   AppName
   Name
   Id

);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> Apps = "apps"
    @Apps
        => AppName 
    @AppName
        -> Name = "name"
);

// impls of each type
impl_macro!(
    @Apps
        |
        |=> app_name -> AppName = appname_str
    @AppName
        |=> name -> Name
        |
);

exec!(Apps);
exec!(AppName);
exec!(Name);
exec!(Id);
