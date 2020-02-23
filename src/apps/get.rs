//! Access the Apps portion of the Heroku API
imports!();
use crate::client::GetQueryBuilder;

// Declaration of types representing the various items under apps
new_type!(
   Apps
   Name
   Id

);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> Apps = "apps"
    @Apps
        -> Name = "name"
        -> Id = "id"
);

// impls of each type
impl_macro!(
    @Apps
        |=> name -> Name
        |=> id -> Id
        |
);

exec!(Apps);
exec!(Name);
exec!(Id);
