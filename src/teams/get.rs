//! Access the Teams portion of the Heroku API
imports!();
use crate::client::GetQueryBuilder;

// Declaration of types representing the various items under Teams
new_type!(
   Teams
   TeamName
   Name
   Id

);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> Teams = "teams"
    @Teams
        => TeamName
    @TeamName
        -> Name = "name"
);

// impls of each type
impl_macro!(
    @Teams
        |
        |=> team_name -> TeamName = teamname_str
    @TeamName
        |=> name -> Name
        |
);

exec!(Teams);
exec!(TeamName);
exec!(Name);
exec!(Id);
