//! Access the Teams portion of the Heroku API
imports!();
use crate::client::GetQueryBuilder;

// Declaration of types representing the various items under Teams
new_type!(
   Teams
   Name
   Id

);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> Teams = "teams"
    @Teams
        -> Name = "name"
        -> Id = "id"
);

// impls of each type
impl_macro!(
    @Teams
        |=> name -> Name
        |=> id -> Id
        |
);

exec!(Teams);
exec!(Name);
exec!(Id);
