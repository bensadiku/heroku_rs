//! Access the Teams portion of the Heroku API
imports!();
use crate::client::GetQueryBuilder;

// Declaration of types representing the various items under Teams
new_type!(
   Teams
   Team
   Addons

);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> Teams = "teams"
    @Teams
        => Team
    @Team
        -> Addons = "addons"
);

// impls of each type
impl_macro!(
    @Teams
        |
        |=> team_name -> Team = team_name_str
        |=> team_id -> Team = team_id_str
    @Team
        |=> team_addons -> Addons
        |
);

exec!(Teams);
exec!(Team);
exec!(Addons);
