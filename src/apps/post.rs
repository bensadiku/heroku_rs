//!Create a new app.
imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Apps
);

from!(
    @PostQueryBuilder
        -> Apps = "apps"
);

impl_macro!(
    @Apps
        |
);

exec!(Apps);
