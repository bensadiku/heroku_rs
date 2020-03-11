 //!Create new /teams implementation.
imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Teams
    Team
    TeamMember
);

from!(
    @PostQueryBuilder
        -> Teams = "teams"
    @Teams
        => Team
    @Team
        -> TeamMember = "members"    
);

impl_macro!(
    @Teams
        |
        |=> team_name -> Team = team_name_str
        |=> team_id -> Team = team_id_str
    @Team
        |=> team_member ->  TeamMember
        |
    @TeamMember
        |
);

exec!(Teams);
exec!(Team);
exec!(TeamMember);
